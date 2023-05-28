# 技术文档

## 网页全栈
网页使用了 Sveltekit 框架，使用服务端渲染，提供了更好的 SEO 体验和更快的加载速度。并且提供 API 断点来做 API 转移。
前端使用了 Material UI 设计语言，提供一个前期设计好的界面 prototype。

要编译网页，需要安装 Node.js 和 npm，然后在项目根目录下运行 `npm install` 安装依赖，再运行 `npm run build` 编译网页。

默认使用 Node Server 运行网页，如果要使用其他服务器，需要将 `./build` 目录下的文件复制到服务器的根目录下。

## API 后端处理
### API 框架
API 使用了 Python 的 FastAPI 框架，使用了 Uvicorn 提供的 ASGI 服务器。

### 问题处理
在获得了用户的请求后，Node 后端会先检查用户是否已经登录。如果已经登录，会检查用户是否有权限访问该 API。

当确认了用户有权限访问该 API 后，会将请求转发到 API 后端。

API 后端使用 Agents 模式对大语言的 API 进行了封装，通过 Prompt 的方式给大语言模型提供多个工具函数，包括：
- `query_sql_db`：通过生成的语句查询 SQL 数据库
- `schema_sql_db`：输入表名查询 SQL 数据库的表结构
- `list_tables_sql_db`：查询 SQL 数据库的所有表
- `query_checker_sql_db`：检查 SQL 查询语句是否合法

该模型将用户的请求转换为 SQL 语句，并且使用 `query_checker_sql_db` 检查 SQL 语句是否合法，如果合法，使用 `query_sql_db` 查询数据库，
如果不合法，返回错误信息，并且尝试使用 `schema_sql_db` 和 `list_tables_sql_db` 返回表结构和表名。
当得到了查询结果后，再将结果转化为自然语言返回给用户。

### 语言模型
语言模型目前使用了 text-davinci-003，该模型是 OpenAI 的 GPT-3 模型，是补全模型。

### 运行方法
要运行 API 后端，需要安装 Python 3.8+ 和 pip，然后在项目根目录下运行 `pip install -r requirements.txt` 安装依赖，
再运行 `uvicorn main:app --reload` 运行 API 后端。

## 守护进程
守护进程使用了 pm2 ，可以在后台运行 API 后端和网页前端。

## 数据转移
因为原数据使用的是 ElasticSearch，而新数据使用的是 Postgres 数据库，所以需要将原数据转移到新数据上。

这里选择了 Rust 语言，使用了 sqlx (SQL 接口) 和 tokio (异步 Runtime) 以及 serde (序列化及反序列化) 来实现快速并且安全的数据转移。

选择 Rust 语言的原因是因为 Rust 语言提供了内存安全和线程安全，可以保证程序的安全性，与此同时实现安全的方法并不是通过 GC，
而是通过所有权系统，所以 Rust 语言的性能非常好，没有 GC 的开销。