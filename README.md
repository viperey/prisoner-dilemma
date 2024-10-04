# Prisoner's Dilemma

Rust implementation of the Prisoner's Dilemma.
Inspired by [Veritasium's video](https://www.youtube.com/watch?v=mScpHTIi-kM).


## TODO

- [ ] StrategyGradual (fix)
- [ ] Iterate the code to make it more Rust idiomatic
- [ ] Fix gradual
- [ ] Moar strategies
- [ ] Population
- [ ] Decay of strategies
- [ ] Add every known strategies
- [ ] Test every the strategy individually
- [ ] Make a GUI

## Pending Strategies

- https://github.com/Axelrod-Python/Axelrod/blob/dev/axelrod/strategies/_strategies.py
- **mem2**: Complex strategy shifting between all_d, tit_for_tat, and tf2t based on recent outcomes and pays attention to memory, adjusting strategy accordingly.
- Probability p Cooperator - Unlikely: Not directly represented
- Suspicious Tit for Tat (STFT)
- Imperfect TFT (ImpTFT)
- Omega Tit for Tat (ΩTFT)
- Discriminating Altruist (DA)
- n-Pavlov
- Adaptive Pavlov (APavlov)
- Reactive (R(y,p,q))
- Memory-one (S(p,q,r,s))
- Zero Determinant (ZD)
- Equalizer (SET-n)
- Extortionary (Extort-n)
- Generous (Gen-n)
- Good (GOOD)


## Bibliography

- https://en.wikipedia.org/wiki/Prisoner%27s_dilemma
- https://scriptim.github.io/PrisonersDilemma/?s1=RAND&s2=RAND&i=344#game
- https://github.com/Scriptim/PrisonersDilemma
- https://www.investopedia.com/terms/i/iterated-prisoners-dilemma.asp
- https://www.jstor.org/stable/173932
- https://www.jstor.org/stable/173638
- https://www.science.org/doi/10.1126/science.7466396
- https://plato.stanford.edu/entries/prisoner-dilemma/index.html#return
- https://plato.stanford.edu/entries/prisoner-dilemma/strategy-table.html
