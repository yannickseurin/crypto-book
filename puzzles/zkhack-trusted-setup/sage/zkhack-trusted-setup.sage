from sage.groups.generic import order_from_multiple

# constructing E1 and E2

p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
F1 = GF(p)
a1 = 0
b1 = 4
E1 = EllipticCurve(F1, [a1, b1])

R.<x> = PolynomialRing(F1)
F2.<u> = F1.extension(x^2+1)
a2 = 0
b2 = 4*(1+u)
E2 = EllipticCurve(F2, [a2, b2])

# G1 is _ts1[0] in the puzzle
G1 = E1(0x0F99F411A5F6C484EC5CAD7B9F9C0F01A3D2BB73759BB95567F1FE4910331D32B95ED87E36681230273C9A6677BE3A69, \
        0x12978C5E13A226B039CE22A0F4961D329747F0B78350988DAB4C1263455C826418A667CA97AC55576228FC7AA77D33E5)
n1 = G1.order()
L1 = list(n1.factor())
print("The factorization of G1's order n1 is:")
for l in L1:
    print(l)
# n1 = 3 * 11 * 10177 * 859267 * 52437899 * 52435875175126190479447740508185965837690552500527637822603658699938581184513
r = 52435875175126190479447740508185965837690552500527637822603658699938581184513 # large factor, also order of subgroups G1 and G2 of BLS12-381
k1 = (n1/r).round() # round() required to convert k1 from Rational to Integer
print("The bit length of k1 = n1/r is:")
print(numerical_approx(log(k1, 2)))

# H1 is _ts1[1] in the puzzle, and we have H1 = s * G1        
H1 = E1(0x16C2385B2093CC3EDBC0F2257E8F23E98E775F8F6628767E5F4FC0E495285B95B1505F487102FE083E65DC8E9E3A9181, \
        0x0F4B73F63C6FD1F924EAE2982426FC94FBD03FCEE12D9FB01BAF52BE1246A14C53C152D64ED312494A2BC32C4A3E7F9A)
# r * G1 and r * H1 are both in a subgroup of order k1 = 3 * 11 * 10177 * 859267 * 52437899 and s * (r * G1) = r * (s * G1) = r * H1
# hence we can find s mod k1 by solving the DL of r * H1 in base r * G1
s1 = discrete_log(r*H1, r*G1, operation='+')
print("s mod k1 =")
print(s1)

# G2 is _ts2[0] in the puzzle
G2 = E2(0x1173F10AD9F2DBEE8B6C0BB2624B05D72EEC87925F5C3633E2C000E699A580B842D3F35AF1BE77517C86AEBCA1130AE4 \
      + 0x0434043A97DA28EF7100AE559167FC613F057B85451476ABABB27CFF0238A32831A0B4D14BA83C4F97247C8AC339841F * u, \
        0x0BEBEC70446CB91BB3D4DC5C8412915E99D612D8807C950AB06BC41583F528FDA9F42EC0FE7CD2991638187EF44258D3 \
      + 0x19528E3B5C90C73A7092BB9AFDC73F86C838F551CCD9DBBA5CC6244CF76AB3372193DBE5B62383FAAE728728D4C1E649 * u)

# n2 = G2.order() # Too long!

E2_order = E2.cardinality()
c2 = E2_order/r
L2 = list(c2.factor())
print("The factorization of cofactor c2 = |E2|/r is:")
for l in L2: 
    print(l)

L2.insert(5, (r,1))
print("The factorization of E2's order is:")
for l in L2:
    print(l)

n2 = order_from_multiple(G2, E2_order, factorization=L2, operation='+')
L3 = list((n2/r).factor())
print("The factorization of n2/r is:")
for l in L3:
    print(l)
L3.insert(5, (r,1))
print("The factorization of G2's order n2 is:")
for l in L3:
    print(l)

rp = 402096035359507321594726366720466575392706800671181159425656785868777272553337714697862511267018014931937703598282857976535744623203249
k2 = (n2/(r*rp)).round()
print("The bit length of k2 = n2/(r*m) is:")
print(numerical_approx(log(k2, 2)))

# H2 is _ts2[0] in the puzzle
H2 = E2(0x165830F15309C878BFE6DD55697860B8823C1AFBDADCC2EF3CD52B56D4956C05A099D52FE4545816830C525F5484A5FA \
      + 0x179E34EB67D9D2DD32B224CDBA57D4BB7CF562B4A3E33382E88F33882D91663B14738B6772BF53A24653CE1DD2BFE2FA * u, \
        0x150598FC4225B44437EC604204BE06A2040FD295A28230B789214B1B12BF9C9DAE6F3759447FD195E92E2B42E03B5006 \
      + 0x12E23B19E117418C568D4FF05B7824E5B54673C3C08D8BCD6D8D107955287A2B075100A51C81EBA44BF5A1ABAD4764A8 * u)

s2 = discrete_log(r*rp*H2, r*rp*G2, ord=k2, operation='+')
print("s mod k2 =")
print(s2)

s12 = crt([s1, s2], [k1, k2])
print("s mod k1 * k2 =")
print(s12)

k = k1 * k2
print("The bit length of k = k1 * k2 is:")
print(numerical_approx(log(k, 2)))

for i in range(2^13):
	s = i*k + s12
	if s * G1 == H1:
		print("discrete log found:")
		print(s)
		break 
