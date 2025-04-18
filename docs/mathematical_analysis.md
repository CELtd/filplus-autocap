# Synthesis of FIL+ Allocator Analysis

This document presents a synthesis of key findings from the [technical analysis of a programmable FIL+ allocator](https://www.overleaf.com/read/pmwfhbkfvjjv#1a46a4), denoted Â, which distributes DataCap to Storage Providers (SPs) based on their declared deal revenue, $r_i$. The study explores the incentives for SP participation and the conditions under which honest reporting of deal revenue is encouraged.

## SP Participation Incentives

### Immediate Reward

- An SP finds it immediately beneficial to participate in the allocator auction if their expected immediate reward  
  $\Delta G_t^i(r_i) = G_t^i(r_i) - G_t^i(r_i = 0)$  
  is non-negative. That is, the gain from participating outweighs the gain from not participating.
  
- This immediate reward depends on:
  - the deal revenue fee, $\gamma$,
  - the block reward, $b_t$, and
  - the change in block-winning probability with and without participation,  
    $Pwin_i(r_i) - Pwin_i(r_i = 0)$.

- The block-winning probability is influenced by the SP’s Quality Adjusted Power (QAP) and the declared revenues of all participating SPs.

- In highly competitive environments (i.e., high $c_i$, the total declared revenues of other SPs), the immediate reward can become negative and decrease as $r_i$ increases. In contrast, with less competition, it may be advantageous for an SP to participate depending on the value of $\gamma$.

- Notably, there may exist a range where declaring a **deal revenue lower than the actual user-paid fee** yields a net positive immediate reward.

- A necessary condition for non-negative immediate reward is:  
  $$\frac{Pwin_i(r_i) - Pwin_i(r_i = 0)}{r_i} \ge \frac{\gamma}{b_t}.$$

- The **maximum declarable deal revenue**, $r_i^{max}$, for which the immediate reward remains non-negative, is influenced by:
  - the datacap-to-QAP conversion factor ($d$),
  - the block reward ($b_t$),
  - the deal revenue fee ($\gamma$),
  - the total QAP of other SPs ($QAP_{tot,i}^0$),
  - the SP’s own initial QAP ($QAP_i^0$), and
  - the level of competition ($c_i$).

  As $c_i$ increases, the range of profitable $r_i$ values shrinks. For lower competition, $r_i^{max}$ decreases with increasing QAP share of the SP.

### Long-Term Investment Strategy

- Even if immediate rewards are negative, participating in Â can be a **strategic long-term investment**, as it boosts an SP’s QAP, improving future rewards.

- If Â becomes the dominant allocator of DataCap, SPs that avoid participation risk gradually losing their share of the total allocation.

- Simulations show that an SP who participates in Â—even while reporting honestly—may initially earn less than a non-participating (greedy) SP. However, over time, the participating SP can **catch up and surpass** the greedy one in cumulative gain. The breakeven point depends on multiple factors.

## Honest vs. Strategic Reporting

- SPs may have an incentive to declare a deal revenue $r_i$ that differs from the actual user-paid fee $u_i$.

- Declaring a higher $r_i$ increases the likelihood of winning DataCap, but also raises the deal revenue fee, $\gamma \cdot r_i$.

- The net expected gain from misreporting is:  
  $$\Delta G_t^i(r_i) = -\gamma(r_i - u_i) + b_t \cdot [Pwin_i(r_i) - Pwin_i(u_i)].$$

- Simulations suggest that depending on market conditions (e.g., $c_i$) and the fee $\gamma$, SPs may be incentivized to misreport.

- Appropriately tuning $\gamma$ may discourage over-reporting. In some cases, SPs may even benefit from **under-reporting** (declaring $r_i < u_i$), though this limits DataCap and slows long-term revenue growth.

- Ultimately, the long-term loss from over-reporting (i.e., $r_i > u_i$) may outweigh any short-term gain, nudging SPs toward SPs toward honest reporting ($r_i = u_i$).

---

**In summary**, this analysis highlights the intricate balance between short-term incentives and long-term strategy for SPs in the FIL+ allocator system. Participation and honest behavior are shaped by the deal revenue fee, market competition, and each SP’s position in the ecosystem.


