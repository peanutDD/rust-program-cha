//! # Eq 和 PartialEq 性能基准测试
//!
//! 本文件包含各种 Eq 和 PartialEq 实现的性能基准测试

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// 测试用的数据结构

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SimpleStruct {
    id: u32,
    name: String,
}

#[derive(Debug, Clone)]
struct OptimizedStruct {
    id: u32,           // 最可能不同的字段
    category: u8,      // 快速比较的字段
    name: String,      // 中等成本的字段
    data: Vec<u8>,     // 最昂贵的字段
}

impl PartialEq for OptimizedStruct {
    fn eq(&self, other: &Self) -> bool {
        // 按照从快到慢的顺序比较
        self.id == other.id &&
        self.category == other.category &&
        self.data.len() == other.data.len() &&  // 长度检查很快
        self.name == other.name &&
        self.data == other.data  // 最昂贵的比较放最后
    }
}

impl Eq for OptimizedStruct {}

impl Hash for OptimizedStruct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.category.hash(state);
        self.name.hash(state);
        self.data.hash(state);
    }
}

#[derive(Debug, Clone)]
struct UnoptimizedStruct {
    id: u32,
    category: u8,
    name: String,
    data: Vec<u8>,
}

impl PartialEq for UnoptimizedStruct {
    fn eq(&self, other: &Self) -> bool {
        // 错误的顺序：昂贵的比较在前
        self.data == other.data &&
        self.name == other.name &&
        self.category == other.category &&
        self.id == other.id
    }
}

impl Eq for UnoptimizedStruct {}

impl Hash for UnoptimizedStruct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.category.hash(state);
        self.name.hash(state);
        self.data.hash(state);
    }
}

// 缓存 Hash 的结构
use std::cell::Cell;

#[derive(Debug)]
struct CachedHashStruct {
    data: Vec<u8>,
    cached_hash: Cell<Option<u64>>,
}

impl CachedHashStruct {
    fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            cached_hash: Cell::new(None),
        }
    }
    
    fn compute_hash(&self) -> u64 {
        if let Some(hash) = self.cached_hash.get() {
            return hash;
        }
        
        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        let hash = hasher.finish();
        self.cached_hash.set(Some(hash));
        hash
    }
}

impl Clone for CachedHashStruct {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            cached_hash: Cell::new(None),  // 不复制缓存
        }
    }
}

impl PartialEq for CachedHashStruct {
    fn eq(&self, other: &Self) -> bool {
        // 首先比较长度（快速）
        if self.data.len() != other.data.len() {
            return false;
        }
        
        // 然后比较 hash（如果已缓存则很快）
        if self.compute_hash() != other.compute_hash() {
            return false;
        }
        
        // 最后比较实际内容
        self.data == other.data
    }
}

impl Eq for CachedHashStruct {}

impl Hash for CachedHashStruct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.compute_hash().hash(state);
    }
}

// 字符串比较优化
#[derive(Debug, Clone)]
struct EfficientStringComparison {
    content: String,
    case_insensitive: bool,
}

impl PartialEq for EfficientStringComparison {
    fn eq(&self, other: &Self) -> bool {
        if self.case_insensitive != other.case_insensitive {
            return false;
        }
        
        if self.case_insensitive {
            // 避免创建新字符串，使用迭代器比较
            self.content.len() == other.content.len() &&
            self.content.chars().zip(other.content.chars())
                .all(|(a, b)| a.to_lowercase().eq(b.to_lowercase()))
        } else {
            self.content == other.content
        }
    }
}

#[derive(Debug, Clone)]
struct InefficientStringComparison {
    content: String,
    case_insensitive: bool,
}

impl PartialEq for InefficientStringComparison {
    fn eq(&self, other: &Self) -> bool {
        if self.case_insensitive != other.case_insensitive {
            return false;
        }
        
        if self.case_insensitive {
            // 低效：创建新字符串
            self.content.to_lowercase() == other.content.to_lowercase()
        } else {
            self.content == other.content
        }
    }
}

// 基准测试函数

fn bench_simple_equality(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_equality");
    
    let items: Vec<SimpleStruct> = (0..1000)
        .map(|i| SimpleStruct {
            id: i,
            name: format!("item_{}", i),
        })
        .collect();
    
    group.bench_function("equal_items", |b| {
        b.iter(|| {
            let item1 = &items[0];
            let item2 = &items[0];
            black_box(item1 == item2)
        })
    });
    
    group.bench_function("different_items", |b| {
        b.iter(|| {
            let item1 = &items[0];
            let item2 = &items[1];
            black_box(item1 == item2)
        })
    });
    
    group.finish();
}

fn bench_optimized_vs_unoptimized(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_vs_unoptimized");
    
    let data = vec![1u8; 1000];
    
    let optimized_items: Vec<OptimizedStruct> = (0..100)
        .map(|i| OptimizedStruct {
            id: i,
            category: (i % 10) as u8,
            name: format!("item_{}", i),
            data: data.clone(),
        })
        .collect();
    
    let unoptimized_items: Vec<UnoptimizedStruct> = (0..100)
        .map(|i| UnoptimizedStruct {
            id: i,
            category: (i % 10) as u8,
            name: format!("item_{}", i),
            data: data.clone(),
        })
        .collect();
    
    // 测试不同 ID 的情况（应该快速退出）
    group.bench_function("optimized_different_id", |b| {
        b.iter(|| {
            let item1 = &optimized_items[0];
            let item2 = &optimized_items[1];
            black_box(item1 == item2)
        })
    });
    
    group.bench_function("unoptimized_different_id", |b| {
        b.iter(|| {
            let item1 = &unoptimized_items[0];
            let item2 = &unoptimized_items[1];
            black_box(item1 == item2)
        })
    });
    
    // 测试相同项的情况
    group.bench_function("optimized_same_item", |b| {
        b.iter(|| {
            let item1 = &optimized_items[0];
            let item2 = &optimized_items[0];
            black_box(item1 == item2)
        })
    });
    
    group.bench_function("unoptimized_same_item", |b| {
        b.iter(|| {
            let item1 = &unoptimized_items[0];
            let item2 = &unoptimized_items[0];
            black_box(item1 == item2)
        })
    });
    
    group.finish();
}

fn bench_hash_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_operations");
    
    let data = vec![1u8; 1000];
    
    let simple_item = SimpleStruct {
        id: 1,
        name: "test".to_string(),
    };
    
    let cached_item = CachedHashStruct::new(data.clone());
    
    group.bench_function("simple_hash", |b| {
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            simple_item.hash(&mut hasher);
            black_box(hasher.finish())
        })
    });
    
    group.bench_function("cached_hash_first_time", |b| {
        b.iter(|| {
            let item = CachedHashStruct::new(data.clone());
            let mut hasher = DefaultHasher::new();
            item.hash(&mut hasher);
            black_box(hasher.finish())
        })
    });
    
    group.bench_function("cached_hash_subsequent", |b| {
        // 预先计算一次 hash
        cached_item.compute_hash();
        
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            cached_item.hash(&mut hasher);
            black_box(hasher.finish())
        })
    });
    
    group.finish();
}

fn bench_string_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_comparison");
    
    let efficient_item1 = EfficientStringComparison {
        content: "Hello World Test String".to_string(),
        case_insensitive: true,
    };
    
    let efficient_item2 = EfficientStringComparison {
        content: "HELLO WORLD TEST STRING".to_string(),
        case_insensitive: true,
    };
    
    let inefficient_item1 = InefficientStringComparison {
        content: "Hello World Test String".to_string(),
        case_insensitive: true,
    };
    
    let inefficient_item2 = InefficientStringComparison {
        content: "HELLO WORLD TEST STRING".to_string(),
        case_insensitive: true,
    };
    
    group.bench_function("efficient_case_insensitive", |b| {
        b.iter(|| {
            black_box(efficient_item1 == efficient_item2)
        })
    });
    
    group.bench_function("inefficient_case_insensitive", |b| {
        b.iter(|| {
            black_box(inefficient_item1 == inefficient_item2)
        })
    });
    
    group.finish();
}

fn bench_collection_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("collection_operations");
    
    let items: Vec<SimpleStruct> = (0..1000)
        .map(|i| SimpleStruct {
            id: i,
            name: format!("item_{}", i),
        })
        .collect();
    
    // HashMap 插入
    group.bench_function("hashmap_insert", |b| {
        b.iter(|| {
            let mut map: HashMap<SimpleStruct, usize> = HashMap::new();
            for (i, item) in items.iter().enumerate().take(100) {
                map.insert(item.clone(), i);
            }
            black_box(map)
        })
    });
    
    // HashMap 查找
    let mut map: HashMap<SimpleStruct, usize> = HashMap::new();
    for (i, item) in items.iter().enumerate() {
        map.insert(item.clone(), i);
    }
    
    group.bench_function("hashmap_lookup", |b| {
        b.iter(|| {
            for item in items.iter().take(100) {
                black_box(map.get(item));
            }
        })
    });
    
    // HashSet 插入
    group.bench_function("hashset_insert", |b| {
        b.iter(|| {
            let mut set: HashSet<SimpleStruct> = HashSet::new();
            for item in items.iter().take(100) {
                set.insert(item.clone());
            }
            black_box(set)
        })
    });
    
    // Vec 查找（线性搜索）
    group.bench_function("vec_contains", |b| {
        b.iter(|| {
            let target = &items[50];
            black_box(items.contains(target))
        })
    });
    
    group.finish();
}

fn bench_data_size_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_size_impact");
    
    let sizes = [10, 100, 1000, 10000];
    
    for size in sizes.iter() {
        let data = vec![1u8; *size];
        
        let item1 = OptimizedStruct {
            id: 1,
            category: 1,
            name: "test".to_string(),
            data: data.clone(),
        };
        
        let item2 = OptimizedStruct {
            id: 1,
            category: 1,
            name: "test".to_string(),
            data: data.clone(),
        };
        
        let item3 = OptimizedStruct {
            id: 2,  // 不同的 ID
            category: 1,
            name: "test".to_string(),
            data: data.clone(),
        };
        
        group.bench_with_input(
            BenchmarkId::new("equal_items", size),
            size,
            |b, _| {
                b.iter(|| black_box(item1 == item2))
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("different_id", size),
            size,
            |b, _| {
                b.iter(|| black_box(item1 == item3))
            },
        );
    }
    
    group.finish();
}

fn bench_hash_consistency_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_consistency");
    
    let items: Vec<SimpleStruct> = (0..100)
        .map(|i| SimpleStruct {
            id: i,
            name: format!("item_{}", i),
        })
        .collect();
    
    group.bench_function("hash_consistency_verification", |b| {
        b.iter(|| {
            for i in 0..items.len() {
                for j in 0..items.len() {
                    if items[i] == items[j] {
                        let mut hasher1 = DefaultHasher::new();
                        items[i].hash(&mut hasher1);
                        let hash1 = hasher1.finish();
                        
                        let mut hasher2 = DefaultHasher::new();
                        items[j].hash(&mut hasher2);
                        let hash2 = hasher2.finish();
                        
                        black_box(hash1 == hash2);
                    }
                }
            }
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_simple_equality,
    bench_optimized_vs_unoptimized,
    bench_hash_operations,
    bench_string_comparison,
    bench_collection_operations,
    bench_data_size_impact,
    bench_hash_consistency_check
);

criterion_main!(benches);