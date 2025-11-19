/// 一个简单的计数器结构体
pub struct Counter {
    value: i32,
}

impl Counter {
    /// 创建一个新的计数器，初始值为0
    pub fn new() -> Self {
        Counter { value: 0 }
    }
    
    /// 获取当前值
    pub fn get(&self) -> i32 {
        self.value
    }
    
    /// 增加值
    pub fn increment(&mut self) {
        self.value += 1;
    }
    
    /// 增加指定数值
    pub fn add(&mut self, n: i32) {
        self.value += n;
    }
    
    /// 重置计数器
    pub fn reset(&mut self) {
        self.value = 0;
    }
    
    /// 检查是否为偶数
    pub fn is_even(&self) -> bool {
        self.value % 2 == 0
    }
}


// src/counter.rs (续)

// 测试模块
#[cfg(test)]  // 这确保测试代码只在运行测试时编译[1,3](@ref)
mod tests {
    use super::*;  // 引入外部函数和结构体[1](@ref)

    // 测试新建计数器
    #[test]  // 标记这是一个测试函数[1](@ref)
    fn test_new_counter() {
        let counter = Counter::new();
        assert_eq!(counter.get(), 0);  // 验证初始值为0[1](@ref)
    }
    
    // 测试增加功能
    #[test]
    fn test_increment() {
        let mut counter = Counter::new();
        counter.increment();
        assert_eq!(counter.get(), 1);
        counter.increment();
        assert_eq!(counter.get(), 2);
    }
    
    // 测试添加数值
    #[test]
    fn test_add() {
        let mut counter = Counter::new();
        counter.add(5);
        assert_eq!(counter.get(), 5);
        counter.add(-2);  // 测试负数
        assert_eq!(counter.get(), 3);
    }
    
    // 测试重置功能
    #[test]
    fn test_reset() {
        let mut counter = Counter::new();
        counter.add(10);
        counter.reset();
        assert_eq!(counter.get(), 0);
    }
    
    // 测试奇偶判断
    #[test]
    fn test_is_even() {
        let mut counter = Counter::new();
        assert!(counter.is_even(), "0应该是偶数");  // 添加自定义错误消息[4](@ref)
        
        counter.increment();
        assert!(!counter.is_even(), "1应该是奇数");
        
        counter.increment();
        assert!(counter.is_even(), "2应该是偶数");
    }
    
    // 测试多个操作组合
    #[test]
    fn test_multiple_operations() {
        let mut counter = Counter::new();
        counter.add(3);
        counter.increment();
        counter.add(2);
        assert_eq!(counter.get(), 6, "3 + 1 + 2 应该等于6");
    }
    
    // 被忽略的测试（默认不运行）
    #[test]
    // #[ignore = "这是一个耗时测试，平时不需要运行"]  // 标记为忽略的测试[1,11](@ref)
    fn test_expensive_operation() {
        let mut counter = Counter::new();
        for _ in 0..1000 {
            counter.increment();
        }
        assert_eq!(counter.get(), 1000);
    }
    
    // 测试应该panic的情况
    #[test]
    #[should_panic]  // 测试预期会发生panic的情况[4,6](@ref)
    fn test_should_panic() {
        // 这个测试只是为了演示should_panic的用法
        panic!("这是预期的panic!");
    }
}