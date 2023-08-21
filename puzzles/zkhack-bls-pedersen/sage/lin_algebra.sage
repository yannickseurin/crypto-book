r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
Fr = FiniteField(r)
load('sage/data.sage')
M = Matrix(Fr, M)
h = vector(Fr, h)
c = M.solve_left(h)
file = open('sage/coeffs.txt', 'w')
for coeff in c:
    file.write(str(coeff) + '\n')
file.close()
