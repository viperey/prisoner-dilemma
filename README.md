# Prisoner's Dilemma

Rust implementation of the Prisoner's Dilemma.
Inspired by the amazing [Veritasium's video](https://www.youtube.com/watch?v=mScpHTIi-kM).
  
Disclaimer: project mainly used to learning Rust language and play around this beautiful problem. Library not mature for production use (yet?).

## What's missing?

- [ ] Add every known strategies, along with their tests.
- [ ] Make a GUI/Cli. Something to allow to play with it.
- [ ] Constant data. Pure constant data. Even if that implies lifetime annotations. Walk that mille.
- [ ] Generate populations. Deterministic.
- [ ] Game of life alike simulation. Meaning: make the prisoners play aginst the neighbors.


## Strategies to add

- https://github.com/Axelrod-Python/Axelrod/blob/dev/axelrod/strategies/_strategies.py
- **mem2**: Complex strategy shifting between all_d, tit_for_tat, and tf2t based on recent outcomes and pays attention to memory, adjusting strategy accordingly.
- Omega Tit for Tat (ΩTFT)
- n-Pavlov
- Adaptive Pavlov (APavlov)
- Memory-one (S(p,q,r,s))
- Zero Determinant (ZD)
- Generous (Gen-n)


## Bibliography

- https://thedecisionlab.com/reference-guide/psychology/prisoners-dilemma
- https://en.wikipedia.org/wiki/Prisoner%27s_dilemma
- https://scriptim.github.io/PrisonersDilemma/?s1=RAND&s2=RAND&i=344#game
- https://github.com/Scriptim/PrisonersDilemma
- https://www.investopedia.com/terms/i/iterated-prisoners-dilemma.asp
- https://www.jstor.org/stable/173932
- https://www.jstor.org/stable/173638
- https://www.science.org/doi/10.1126/science.7466396
- https://plato.stanford.edu/entries/prisoner-dilemma/index.html#return
- https://plato.stanford.edu/entries/prisoner-dilemma/strategy-table.html
