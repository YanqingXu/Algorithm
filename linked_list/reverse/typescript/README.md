# TypeScript 实现

## 环境要求

- Node.js >= 16.0.0
- npm >= 7.0.0

## 安装依赖

```bash
npm install
```

## 运行方式

### 方式1：直接运行 (推荐)
```bash
npm run dev
```

### 方式2：编译后运行
```bash
npm run build
npm start
```

### 方式3：使用ts-node
```bash
npx ts-node reverse_k_group_with_constraints.ts
```

## 项目结构

```
typescript/
├── package.json              # 项目配置和依赖
├── tsconfig.json            # TypeScript编译配置
├── reverse_k_group_with_constraints.ts  # 主要算法实现
└── README.md               # 说明文档
```

## 特性

- ✅ 完整的TypeScript类型注解
- ✅ 严格的编译选项
- ✅ Node.js环境支持
- ✅ 开发和生产环境脚本
- ✅ 源码映射支持

## 已知问题

- 测试用例2的逻辑需要进一步调试（分隔符插入条件）

## 依赖说明

### 开发依赖
- `typescript`: TypeScript编译器
- `ts-node`: 直接运行TypeScript文件
- `@types/node`: Node.js类型定义
