//! Sample deck seeder for onboarding
//!
//! Creates a "Linear Algebra" deck with sample Q&A cards when no decks exist.

use crate::data::models::CardContent;
use crate::DeckManager;
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

/// Create sample linear algebra deck if no decks exist
pub async fn seed_sample_decks(deck_manager: &DeckManager) -> anyhow::Result<()> {
    let decks = deck_manager.get_all_decks().await?;
    if !decks.is_empty() {
        return Ok(());
    }

    let deck_id = deck_manager
        .create_deck(
            "Linear Algebra".to_string(),
            Some("Basic linear algebra concepts and examples".to_string()),
            None,
        )
        .await?;

    let cards = vec![
        card(
            "什么是矩阵？",
            "矩阵（Matrix）是一个按照长方阵列排列的复数或实数集合，通常用于表示线性变换或方程组。例如：\n[1 2]\n[3 4]",
            vec!["基础", "矩阵"],
        ),
        card(
            "什么是行列式（Determinant）？",
            "行列式是一个可以从方阵计算出的标量值，记作 det(A) 或 |A|。它反映了线性变换对面积的缩放因子。\n对于 2×2 矩阵 [a b; c d]，行列式为 ad - bc。",
            vec!["基础", "行列式"],
        ),
        card(
            "矩阵乘法如何计算？",
            "两个矩阵 A (m×n) 和 B (n×p) 相乘，结果 C = AB 是 m×p 矩阵。\nC 的第 i 行第 j 列元素 = A 的第 i 行与 B 的第 j 列的点积。\n注意：矩阵乘法不满足交换律，即 AB ≠ BA（一般情况下）。",
            vec!["矩阵运算", "乘法"],
        ),
        card(
            "什么是特征值和特征向量？",
            "对于方阵 A，如果存在非零向量 v 和标量 λ 使得 Av = λv，则 λ 称为 A 的特征值，v 称为对应的特征向量。\n特征值反映了线性变换在某些方向上的缩放因子。",
            vec!["特征值", "特征向量"],
        ),
        card(
            "什么是逆矩阵？",
            "对于 n×n 方阵 A，如果存在矩阵 B 使得 AB = BA = I（单位矩阵），则 B 称为 A 的逆矩阵，记作 A⁻¹。\n逆矩阵存在的充要条件是 det(A) ≠ 0。\n2×2 矩阵 [a b; c d] 的逆矩阵为 (1/(ad-bc)) × [d -b; -c a]。",
            vec!["矩阵运算", "逆矩阵"],
        ),
        card(
            "什么是线性相关和线性无关？",
            "一组向量 {v₁, v₂, ..., vₙ} 如果存在不全为零的系数 c₁, c₂, ..., cₙ 使得 c₁v₁ + c₂v₂ + ... + cₙvₙ = 0，则称它们线性相关。\n否则称为线性无关。线性无关的向量组构成了向量空间的基。",
            vec!["线性相关", "线性无关", "基"],
        ),
        card(
            "什么是秩（Rank）？",
            "矩阵的秩是矩阵中线性无关的行（或列）的最大数量，记作 rank(A)。\n秩等于行空间的维数，也等于列空间的维数。\n满秩矩阵：rank(A) = min(m, n)。",
            vec!["秩", "线性无关"],
        ),
        card(
            "解线性方程组 Ax = b 的方法？",
            "常用方法：\n1. 高斯消元法（Gaussian Elimination）：通过行变换将增广矩阵化为行阶梯形\n2. 克拉默法则（Cramer's Rule）：xᵢ = det(Aᵢ)/det(A)\n3. 矩阵求逆法：x = A⁻¹b（当 A 可逆时）\n4. LU 分解法",
            vec!["方程组", "高斯消元"],
        ),
        card(
            "什么是正交矩阵？",
            "如果一个 n×n 实矩阵 Q 满足 QᵀQ = QQᵀ = I，则 Q 是正交矩阵。\n正交矩阵的性质：\n- Q⁻¹ = Qᵀ\n- 列（行）向量组是标准正交基\n- 保持向量的长度和夹角不变\n- det(Q) = ±1",
            vec!["正交矩阵", "正交"],
        ),
        card(
            "什么是向量空间的基（Basis）？",
            "向量空间 V 的一组基是 V 中线性无关且能生成整个 V 的向量集合。\n基中向量的个数称为向量空间的维数（dimension）。\n例如：R² 的标准基是 e₁ = (1,0) 和 e₂ = (0,1)。",
            vec!["基", "维数", "向量空间"],
        ),
    ];

    deck_manager.add_cards(&deck_id, cards).await?;

    Ok(())
}

fn card(front: &str, back: &str, tags: Vec<&str>) -> CardContent {
    let now = Utc::now();
    CardContent {
        id: Uuid::new_v4(),
        front: front.to_string(),
        back: back.to_string(),
        tags: tags.into_iter().map(String::from).collect(),
        media: None,
        custom: HashMap::new(),
        created_at: now,
        modified_at: now,
    }
}
