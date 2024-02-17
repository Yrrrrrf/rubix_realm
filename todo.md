# Algorithms to do

### [Horners Rule](https://en.wikipedia.org/wiki/Horner%27s_method)
The Horner's rule is a method to evaluate polynomials. It is a way to evaluate a polynomial in a way that is efficient and **avoids the use of powers of x**. It is particularly useful for evaluating polynomials in computer programs.

The Horner's rule is based on the following observation:
```
P(x) = a0 + a1*x + a2*x^2 + ... + an*x^n
     = a0 + x*(a1 + x*(a2 + ... + x*(an-1 + an*x)))
```
The Horner's rule is a way to evaluate the polynomial in the second form, which is more efficient than the first form.