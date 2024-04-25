# rsfrp
implement frp with rust


任务拆分:
1. 命令行参数处理
2. 配置文件解析
3. tokio异步编程  实现一个nc
4. 实现一个简易版tcp代理
    sshd:22 <--> server <--> ssh-client



参数设计：
rsfrp server -c rsfrps.toml
rsfrp client -c rsfrps.toml


配置文件设计：