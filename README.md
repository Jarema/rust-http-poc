# rust-http-poc

Development of the app should be possible just after installing Rust. There is no additional environment setup that is needed.

# development

```cargo build``` builds the binary  
```cargo run``` runs the application

# Troubleshooting

If on Linux distros,

 ```error: linking with `cc` failed: exit code: 1``` 
 
  happens, 
  
  ```apt install gcc-multilib``` 
  
  solves the issue.