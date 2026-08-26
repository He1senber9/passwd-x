# 加密保险库文件格式（v1）

本文档描述 `core/` 生成的加密保险库文件格式。版本化设计为未来升级 KDF 参数或迁移条目级加密留出空间。

## 布局

整体文件为二进制，多字节整数一律小端（LE）：

| 偏移 | 长度（字节） | 字段 | 说明 |
| --- | --- | --- | --- |
| 0 | 4 | magic | 固定 `PWDX` |
| 4 | 4 | version | 文件格式版本，当前为 1 |
| 8 | 4 | m_cost_kib | Argon2id 内存成本（KiB） |
| 12 | 4 | t_cost | Argon2id 迭代次数 |
| 16 | 4 | p_cost | Argon2id 并行度 |
| 20 | 16 | salt | KDF 盐 |
| 36 | 24 | wrap_nonce | 包装 DEK 用的 AEAD nonce |
| 60 | 48 | wrapped_dek | 主密钥（MK）加密的 32 字节 DEK + 16 字节标签 |
| 108 | 24 | vault_nonce | 加密保险库用的 AEAD nonce |
| 132 | 可变 | ciphertext | 保险库 JSON 的密文 + 16 字节标签 |

## 密钥体系

1. 主密码 → Argon2id（盐取自文件头）→ 32 字节主密钥（MK）
2. MK 用 AEAD 解出包装后的数据密钥（DEK）
3. DEK 用 AEAD 解密整个保险库 JSON

保险库 JSON 内容（同样带版本号）：

```json
{
  "version": 1,
  "entries": [
    {
      "id": "uuid",
      "title": "...",
      "username": "...",
      "password": "...",
      "urls": ["..."],
      "notes": "...",
      "created_at": 1700000000,
      "updated_at": 1700000000
    }
  ]
}
```

## 安全性质

- AEAD：XChaCha20-Poly1305，nonce 每次保存随机生成（24 字节，碰撞概率可忽略）
- 关联数据绑定：包装 DEK 与加密保险库均以「magic + version + KDF 参数 + 盐 + nonce（+ wrapped_dek）」作为 AAD，头部任何字节被篡改都会导致认证失败
- 每次 `save` 重新生成盐与 nonce，并重新包装 DEK；解锁期间 DEK 保持不变
- 修改主密码等价于用新密码 `save`：仅重新包装 DEK，无需重新加密条目
- 密钥（MK/DEK）使用后立即从内存擦除（`zeroize`）
- 解锁失败统一报「密码错误或数据损坏」，不区分具体认证失败位置

## 参数与限制

- 默认 KDF 参数：m=19456 KiB（19 MiB）、t=2、p=1（OWASP 基线）
- 解析文件时校验参数范围：m ∈ [8, 1_048_576] KiB 且 m ≥ 8·p、t ∈ [1, 32]、p ∈ [1, 8]，防止恶意文件触发巨额内存分配
- 主密码最短 8 字符、最长 1024 字节

## 版本策略

- `version = 1`：单文件整体加密（当前实现）
- 未来升级：读取方拒绝未知版本；引入条目级加密（云同步）时新增版本或独立同步格式，另行设计文档
