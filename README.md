# 🌾 产地农产品产销协同平台

一个基于 Rust/Axum/Tera/SQLite 构建的产地农产品产销协同管理平台，实现从农户收成登记到最终对账结算的全流程数字化管理。

## 功能模块

### 📋 收成登记
- 农户登记农产品收成信息（品种、数量、单价、日期等）
- 支持收成记录的查看、编辑和状态管理
- 状态流转：待入库 → 已入库

### 🏭 仓储入库
- 关联收成记录进行入库操作
- 记录库位、入库数量、仓管员等信息
- 入库后自动更新收成记录状态

### 🛒 分销接单
- 分销渠道下单采购农产品
- 记录渠道信息、联系人、采购数量和价格
- 自动计算订单总金额
- 状态流转：待确认 → 已确认 → 配送中 → 已完成

### 🚚 物流追踪
- 创建物流运输记录，关联分销订单
- 记录司机、车辆、出发和预计到达时间
- 实时追踪运输状态
- 状态流转：待出发 → 运输中 → 已送达

### 💰 对账结算
- 关联分销订单和收成记录创建结算单
- 自动关联农户信息
- 支持多种支付方式
- 状态流转：待支付 → 已支付

### 📊 数据看板
- 全方位统计展示各环节数据
- 收成总量、销售总额、待结算金额等关键指标
- 最近收成登记列表

## 技术栈

| 组件 | 技术 |
|------|------|
| 后端框架 | Axum 0.7 |
| 数据库 | SQLite (rusqlite) |
| 模板引擎 | Tera |
| 认证方式 | Cookie-Session |
| 密码加密 | bcrypt |
| 前端 | 原生 HTML/CSS/JS |
| 容器化 | Docker |

## 项目结构

```
repo/
├── src/
│   ├── main.rs              # 应用入口，路由配置
│   ├── config/              # 配置模块
│   │   ├── app.rs           # 应用状态定义
│   │   ├── database.rs      # 数据库初始化
│   │   └── seed.rs          # 种子数据
│   ├── handlers/            # 请求处理器
│   │   ├── auth.rs          # 认证处理
│   │   ├── harvest.rs       # 收成登记处理
│   │   ├── warehouse.rs     # 仓储入库处理
│   │   ├── distribution.rs  # 分销接单处理
│   │   ├── logistics.rs     # 物流追踪处理
│   │   ├── settlement.rs    # 对账结算处理
│   │   └── home.rs          # 首页和看板处理
│   ├── middleware/           # 中间件
│   │   └── auth.rs          # 认证中间件
│   ├── models/              # 数据模型
│   ├── services/            # 业务逻辑服务
│   └── ...
├── templates/               # Tera 模板
│   ├── base.html            # 基础布局
│   ├── index.html           # 首页
│   ├── partials/            # 公共组件
│   ├── auth/                # 认证页面
│   ├── harvests/            # 收成登记页面
│   ├── warehouse/           # 仓储入库页面
│   ├── distributions/       # 分销接单页面
│   ├── logistics/           # 物流追踪页面
│   ├── settlements/         # 对账结算页面
│   └── dashboard/           # 数据看板页面
├── static/                  # 静态资源
│   ├── css/style.css
│   └── js/main.js
└── Cargo.toml
```

## 快速开始

### 本地开发

```bash
cd repo
cargo run
```

访问 http://localhost:3000

### Docker 部署

```bash
docker build -t farm-produce .
docker run -p 3000:3000 -p 2222:22 farm-produce
```

## 演示账号

| 角色 | 用户名 | 密码 |
|------|--------|------|
| 农户 | zhangdashan | 农户1 |
| 农户 | lixiuying | 农户2 |
| 仓管 | wangjianguo | 仓管1 |
| 分销 | chenminghua | 分销1 |
| 物流 | zhaodelong | 物流1 |
| 管理员 | admin | 管理员 |

## 数据库设计

### 用户表 (users)
- id, username, password_hash, role, name, phone, created_at

### 会话表 (sessions)
- id, user_id, token, created_at

### 收成表 (harvests)
- id, farmer_id, product_name, quantity, unit, price_per_unit, harvest_date, status, description, created_at

### 仓储表 (warehouse_entries)
- id, harvest_id, keeper_id, location, quantity_stored, intake_date, status, notes, created_at

### 分销表 (distributions)
- id, channel_name, contact_person, contact_phone, product_name, quantity, unit_price, total_amount, status, created_at

### 物流表 (logistics)
- id, distribution_id, driver_name, driver_phone, vehicle_plate, departure_time, estimated_arrival, actual_arrival, status, notes, created_at

### 结算表 (settlements)
- id, distribution_id, harvest_id, farmer_id, amount, status, payment_method, paid_at, notes, created_at

## 架构设计

采用 MVC 分层架构：

- **Handlers（控制器）**：处理 HTTP 请求，调用 Service 层，渲染模板
- **Services（业务逻辑）**：封装数据库操作和业务规则
- **Models（数据模型）**：定义数据结构和表单

认证流程：
1. 用户登录 → 验证用户名密码 → 创建 Session Token
2. Token 存入 Cookie（HttpOnly）
3. 每次请求通过 Cookie 中的 Token 查询 Session 获取用户信息
4. 登出时删除 Session 和 Cookie

## 许可证

MIT License
