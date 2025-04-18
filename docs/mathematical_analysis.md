# Mathematical Framework for FIL Allocation

## 1. Aim of this Document

This document aims to analyze the impact of an allocator, $\hat{A}$, within the FIL+ framework that distributes datacap to Storage Providers (SPs) in proportion to their declared deal revenue, $r_i$. The study focuses on understanding the economic incentives behind SP participation and the strategic implications of declaring different deal revenues. Specifically, we aim to address the following key points:

1. **SP Participation Incentives**: Under what conditions is it advantageous for an SP to participate in the datacap allocation mechanism of $\hat{A}$? This analysis will examine:
   * The immediate expected reward of engaging with $\hat{A}$ versus opting out, within a short-term investment framework.
   * The long-term dynamics for an SP that initially finds participation unprofitable, identifying whether and when it becomes advantageous to engage with $\hat{A}$ over time.

2. **Honest vs. Strategic Reporting**: If an SP decides to participate, what incentives exist for reporting an honest deal revenue versus inflating $r_i$ to maximize rewards? Under what conditions does the system encourage truthful behavior?

To answer these questions, we will first introduce the mathematical framework describing SP incentives and the reward structure. We will then analyze the theoretical formulations and simulate potential outcomes to assess the robustness of different SP strategies.

## 2. Introduction

We consider a FIL+ allocator, $\hat{A}$, that distributes datacap to Storage Providers (SPs) in proportion to their declared deal revenues, $r$. In this setting, an SP enters into a storage deal with a user who pays a service fee, $u_i$. However, the SP has the flexibility to declare a deal revenue, $r_i$, which may strategically differ from the actual user-paid fee, $u_i$. The SP's decision to report $r_i$ affects its datacap allocation, influencing its competitive standing and overall profitability.

The expected gain for an SP winning a block while declaring a deal revenue $r_i$, with a user fee $u_i$ and a deal-revenue fee $\gamma$, is given by:

$$G^t_i(r_i) = u_i - \gamma r^t_i + b^t \cdot P^{win,t}_i(r_i)$$

Where $b^t$ will be the block reward at time $t$, and $P^{win,t}_i(r_i)$ is the probability of the i-th SP to win the block. This quantity is defined as:

$$P^{win,t}_i(r_i) = \frac{QAP^0_i + \Delta QAP^t_i(r_i)}{\sum_j QAP^0_j + \Delta QAP^t_j(r_j)}$$

Where $QAP^0_i$ is the Quality Adjusted Power of the i-th SP at time $t-1$ and $\Delta QAP^t_i(r_i)$ is the variation in the Quality Adjusted Power of the i-th SP after considering the extra datacap provided by the allocator for its declared deal revenue $r_i$.

We define, in accordance with the document provided, the extra datacap provided by the allocator $\hat{A}$ to the i-th SP as:

$$\Delta QAP^t_i(r_i) = d \cdot \frac{r^t_i}{\sum_j r^t_j}$$

Where $d$ is a datacap to QAP conversion factor.

To quantify the short-term incentives for an SP to engage with the allocator $\hat{A}$, we evaluate the conditions under which the following inequality holds:

$$\Delta G^t_i = G^t_i(r_i) - G^t_i(r_i = 0) \geq 0$$

If $\Delta G^t_i < 0$ and the SP follows a greedy strategy, it will initially refrain from engaging with $\hat{A}$. However, over time, its share of the total datacap will decrease, leading to a decline in its revenues. At some point, it will become advantageous for the SP to participate in $\hat{A}$ after a certain threshold time $t^*$.

It is therefore crucial to simulate and quantify the expected time horizon $T$ at which an SP with a long-term perspective will outperform a purely greedy allocator:

$$\sum_{t \geq t^*}^T \Delta G^t_i(r_i) \geq \sum_{t \geq t^*}^T \Delta G^t_i(r_i = 0)$$

Furthermore, we analyze whether an SP has an incentive to declare a deal revenue $r_i$ different from the actual user-paid fee $u_i$:

$$\Delta G^t_i = G^t_i(r_i) - G^t_i(r_i = u_i) \geq 0$$

## 3. Analysis

We will first analyze the variation in the probability of winning a block for the i-th SP that decides to engage with $\hat{A}$.

Let us work out $P^{win,t}_i(r_i)$

$$P^{win,t}_i(r_i) = \frac{QAP^0_i + \Delta QAP^t_i(r_i)}{\sum_j QAP^0_j + \Delta QAP^t_j(r_j)}$$

Considering the variation in the QAP:

$$\Delta QAP^t_i(r_i) = d \cdot \frac{r^t_i}{\sum_j r^t_j} = d \cdot \frac{r^t_i}{\sum_{j\neq i} r^t_j + r^t_i} = d \cdot \frac{r^t_i}{c^t_i + r^t_i}$$

Where $c^t_i = \sum_{j\neq i} r^t_j$.

Given that:

$$\sum_j \Delta QAP^t_j(r_j) = \sum_j d \cdot \frac{r^t_j}{\sum_k r_k} = d$$

Then the probability for the i-th SP to win the block will simply be:

$$P^{win,t}_i(r_i) = \frac{QAP^0_i + d \cdot \frac{r^t_i}{r^t_i + c^t_i}}{QAP^0_{tot} + d}$$

Where $QAP^0_{tot} = \sum_j QAP^0_j$

![Figure 1: Winning probability P_i as a function of SP's declared deal revenue r_i. Parameters are: QAP^0_i = 10, QAP^0_{tot} = 100, d = 10, and c_i = 10](docs/images/win-probability.png)

Note that: 
$$\lim_{r_i \to \infty} \Delta QAP^t_i(r_i) = d$$
$$\lim_{r_i \to \infty} P^{win,t}_i(r_i) = \frac{QAP^0_i + d}{(QAP^0_i + QAP^0_{tot,i}) + d}$$

Where
$$QAP^0_{tot,i} = \sum_{j\neq i} QAP^0_j$$

Thus, the probability of winning is bounded from above as a function of the declared deal revenue.

The expected gain of a SP reads:

$$G^t_i(r_i) = u_i - \gamma r^t_i + b^t \cdot P^{win,t}_i(r_i)$$
$$= u_i - \gamma r^t_i + b^t \cdot \frac{QAP^0_i + d \cdot \frac{r^t_i}{r^t_i + c^t_i}}{QAP^0_{tot} + d}$$

Thus, the expected gain depends on several factors such as: the block reward $b^t$; the user fee $u_i$; the declared deal revenue fee $\gamma$; the share of total QAP that the i-th SP owns at time $t = 0$; the total $\Delta QAP_{tot} = d$, which is the datacap to QAP conversion factor; the share of the total $\Delta QAP_{tot} = d$ the i-th is capable of obtaining.

We loosely analyzed the effect of the latter four variables in Figure 2, assuming that with the allocator $\hat{A}$, 10 honest SPs are engaging, each of which has reported a deal revenue $r_j = u_j = 0.1$ FIL.

![Figure 2: Expected gain of the i-th SP as a function of the declared deal revenue normalized by c_i = \sum_{j\neq i} r_j in percentage](https://placeholder-image.com/expected-gain-graph)

From the perspective of the Storage Provider (SP), participating in the allocator auction is beneficial if one of the following conditions holds:

1. The SP immediately earns a higher reward $\Delta G^t(r_i)$ compared to not participating in the allocator's auction.
2. Even if the SP does not gain an immediate reward advantage, participation increases their QAP (Quality-Adjusted Power). This, in turn, can serve as a long-term investment strategy: while the immediate gain might be lower, the increased QAP could lead to greater future rewards.

Let us first consider the immediate reward.

### 3.1 SP Participation Incentives: immediate reward

For a SP it is immediately advantageous to engage with $\hat{A}$ if the expected immediate reward is positive:

$$\Delta G^t_i(r_i) = G^t_i(r_i) - G^t_i(r_i = 0) \geq 0$$
$$= G^t_i(r_i) - u_i - b^t P^{win}_i(r_i = 0) \geq 0$$
$$= -\gamma r_i + b^t \left( P^{win}_i(r_i) - P^{win}_i(r_i = 0) \right) \geq 0$$

The probability of winning will strongly depend on the distribution of QAP and the deal revenues reported by other SPs. In essence, if participating in the Allocator's auction will result in a net increase in the QAP of the SP, then it might be advantageous to participate. If the variation is not so high, then a greedy SP might find it more attractive not to use the Allocator.

![Figure 3: Expected immediate reward \Delta G^t_i of the i-th SP as a function of the declared deal revenue normalized by c_i = \sum_{j\neq i} r_j in percentage](https://placeholder-image.com/immediate-reward-graph)

In Figure 3 we report the analysis of the expected immediate reward $\Delta G^t(r_i)$ of the i-th SP. In this analysis, we assume that the other competitor SPs are honest and agnostic, meaning they'll report a declared deal revenue equal to the user-paid fee. Thus, $c_i$ is a direct measure of the number of competing SP engaging with $\hat{A}$. It can be noticed that, if the competition is high between the SPs (left panel $c_i = 1$, i.e., 10 competing SPs), the instantaneous reward of the i-th SP will always be lower than zero, and it will decrease as the declared deal revenue increases. If the number of competing SPs decreases (right panel $c_i = 0.2$, i.e., two competing SPs) then the i-th SP might find instantaneously advantageous to engage with $\hat{A}$, depending on the deal-revenue fee imposed by $\hat{A}$.

Interestingly, in this case, the i-th SP can even find it more advantageous to declare a deal revenue which is lower than the actual user-paid fee (right panel, for $\gamma = 20\%$).

The condition for which there is a window for the i-th SP of having a net immediate reward is:

$$\frac{P^{win}_i(r_i) - P^{win}_i(r_i = 0)}{r_i} \geq \frac{\gamma}{b^t}$$

By plugging Eq. 10 in Eq. 15, we obtain that the immediate reward for the i-th SP is greater than zero if:

$$0 \leq r_i \leq \frac{d \cdot b^t}{\gamma \cdot (QAP^0_{tot,i} + QAP^0_i + d)} - c_i$$

In Figure 4 we plot the maximum declarable deal revenue for which $\Delta G^t_i(r^{max}_i > 0) = 0$ as a function of the currently owned share of the total available datacap, i.e.

$$r^{max}_i(QAP^0_i) = \frac{d \cdot b^t}{\gamma \cdot (QAP^0_{tot,i} + QAP^0_i + d)} - c_i$$

![Figure 4: Maximum declarable deal revenue r_i as a function of the relative QAP owned by a storage provider plotted for different values of the competitor's participation c_i](https://placeholder-image.com/max-revenue-graph)

As shown in Fig. 4, increasing the number of honest and agnostic competitors engaging with $\hat{A}$ ($c_i$) erodes the possibility of having a declared deal revenue $r_i \geq 0$ such that the immediate reward for the i-th SP is greater than zero ($c_i > 0.3$ in our case and not shown in the figure). For low competitor participation, the maximum $r_i$ for which the SP can experiment an expected immediate reward $\Delta G^t_i(r_i) \geq 0$ is greater than zero and decreases as a function of the initial share $QAP^0_i$ owned by the i-th SP of the total QAP available. The shaded regions indicate ranges where the SP experiences a positive immediate gain by declaring revenue $r_i$ below the user-paid fee $u_i$ (red dotted line).

### 3.2 SP Participation Incentives: long-term investment strategy

As shown in the previous section, it is not guaranteed that, provided the market conditions, the SP will find it immediately more profitable to engage with the allocator $\hat{A}$. However, if we assume that allocator $\hat{A}$ is the only provider in the FIL+ ecosystem of DataCap, this will result in a slow erosion of the SP's owned shares of the total DataCap. Let us assume that we have two different SPs Alice A and Bob B. Both A and B obtain a deal with a client paying $u_i = u_a = u_b$ at time $t = 1\Delta t$, where $\Delta t$ is the timestep between each new block produced. Alice never engages with $\hat{A}$, while Bob decides to engage at time $t = 1$ with the allocator declaring a deal revenue $r_b$. Their expected revenues in time will thus be:

$$G^t_A = u_i + b^t \cdot \sum_{t\geq 1}^N \frac{QAP^0}{Q^0_{tot} + d \cdot t}$$
$$G^t_B = u_i - \gamma \cdot r_b + b^t \cdot \sum_{t\geq 1}^N \frac{QAP^0 + d \cdot \frac{r_b}{r_b + c_b}}{Q^0_{tot} + d \cdot t}$$

Thus, Bob made the right choice if it exists a time $t^* \geq 1$ such that:

$$\Delta G^{t^*}_{A-B} = G^t_B - G^t_B \geq 0$$

Meaning if:

$$\Delta G^{t^*}_{A-B} = -\gamma \cdot r_b + b^t \cdot \sum_{t\geq 1}^{t^*} \frac{d \cdot \frac{r_b}{r_b + c_b}}{Q^0_{tot} + d \cdot t} \geq 0$$

![Figure 5: Analysis of the differences in rewards \Delta G^t_{A-B} between a greedy SP (A) and an SP that has engaged with \hat{A} at time t = 0](https://placeholder-image.com/rewards-difference-graph)

In Fig. 5 we report the trend in time of the differences in returns expected for Bob and Alice. In particular, we have assumed that Bob was honest, declaring a deal revenue equal to the fee the user has paid him. It is, however, interesting to notice that, even if at time $t = 1$, Bob has an expected gain which is less than the greedy strategy of Alice, in time Bob catches up, overcoming Alice. The time for which Bob earns more than Alice depends on several factors. Nevertheless, this demonstrates that, although it may not be instantly advantageous for an SP to engage with $\hat{A}$, over time this can result in a good decision.

### 3.3 Honest vs. Strategic Reporting:

As shown in Sec. 3.1, there might be conditions under which the SP can be incentivized to declare a deal revenue $r_i$ which is different from the user paid fee $u_i$.

To address whether the SP is actually incentivized to declare the false, we need to evaluate the net expected gain of the SP. In particular, in this case, increasing the declared $r_i$ increases the chances of winning the datacap, and thus the block, at the cost of paying a higher deal-revenue fee $\gamma \cdot r_i$. This means we need to evaluate whether the expected immediate reward from declaring the truth is, in fact, advantageous for the SP compared to the alternative:

$$\Delta G^t_i(r_i) = G^t_i(r_i) - G^t_i(u_i) \geq 0$$
$$= G^t_i(r_i) - u_i + \gamma \cdot u_i - b^t P^{win}_i(u_i) \geq 0$$
$$= -\gamma(r_i - u_i) + b^t \left( P^{win}_i(r_i) - P^{win}_i(u_i) \right) \geq 0$$

![Figure 6: Expected immediate reward of a dishonest SP \Delta G^t_i as a function of the declared deal revenue normalized by u_i](https://placeholder-image.com/dishonest-sp-reward-graph)

In Fig. 6 we are plotting the function defined in Eq. 21 under the condition $G(u_i) > G(0)$, depending on the market participation $c_i$, and on the deal revenue fee $\gamma$. As it can be noticed, the SP can be incentivized to report a deal revenue different from the actual deal revenue $u_i$. Nevertheless, if the fee ($\gamma$) is properly set, the SP will be incentivized to declare a deal revenue that is less than the actual revenue. However, this in turn will lead the SP to get less datacap and a slower expected revenue increase as time passes. Therefore, the long-term generated revenues for $r_i > u_i$, together with a lower instantaneous revenue for declaring $r_i > u_i$, might enforce the SP to declare $r_i = u_i$.