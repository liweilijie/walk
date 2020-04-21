# walk
walk dir

功能：查找后缀是`jpg`的文件总个数

## 代码注释


```rust
// 11行
let entry = entry?; // 这条语句会将没有权限等操作都返回。这里处理一下。
```

```rust
    if let Err(e) = walk(env::args().nth(1).unwrap()) {
        println!("error: {}", e);
    }

    /*
    等同于这样:
    match walk(env::args().nth(1).unwrap()) {
        Err(e) => println!("error {}", e),
        Ok(_) => {},
    }
    */
```
