                            * = $0001
; "Main" code
1000   20 57 10   L1000     JSR L1057
1003   20 1C 10             JSR L101C
1006   A2 00                LDX #$00
1008   8E 01 F0             STX $F001
100B   20 31 10             JSR L1031
100E   A2 00                LDX #$00
1010   20 4C 10             JSR L104C
1013   D0 EB                BNE L1000
1015   20 67 10   L1015     JSR L1067
1018   4C 15 10             JMP L1015
101B   00                   BRK

; Subroutine 1
101C   A9 0A      L101C     LDA #$0A
101E   8D 60 20             STA $2060
1021   A2 00                LDX #$00
1023   BD 00 DE   L1023     LDA $DE00,X
1026   45 03                EOR $03
1028   9D 61 20             STA $2061,X
102B   E8                   INX
102C   E0 0F                CPX #$0F
102E   D0 F3                BNE L1023
1030   60                   RTS

; Subroutine 2
1031   AD 02 F0   L1031     LDA $F001
1034   F0 FB                BEQ L1031
1036   20 45 10             JSR L1045
1039   9D 10 DD             STA $DD10,X
103C   9D 71 20             STA $2071,X
103F   E8                   INX
1040   E0 1E                CPX #$1E
1042   D0 ED                BNE L1031
1044   60                   RTS

; Subroutine 3
1045   AD 00 C0   L1045     LDA $C000
1048   CE 01 F0             DEC $F001
104B   60                   RTS

; Subroutine 4
104C   BD 50 DD   L104C     LDA $DD50,X
104F   41 03                EOR ($03,X)
1051   DD 10 DD             CMP $DD10,X
1054   F0 F6                BEQ L104C
1056   60                   RTS

; Subroutine 5
1057   A2 00      L1057     LDX #$00
1059   A9 00      L1059     LDA #$00
105B   9D 10 DD             STA $DD10,X
105E   9D 71 20             STA $2071,X
1061   E8                   INX
1062   E0 1E                CPX #$1E
1064   D0 F3                BNE L1059
1066   60                   RTS

; Subroutine 6
1067   A2 00      L1067     LDX #$00
1069   BD 80 DD   L1069     LDA $DD80,X
106C   F0 12                BEQ L1080
106E   85 25                STA $25
1070   BD 00 07             LDA $0700,X
1073   41 03                EOR ($03,X)
1075   A5 25                LDA $25
1077   45 DE                EOR $DE
1079   9D 71 20             STA $2071,X
107C   E8                   INX
107D   4C 69 10             JMP L1069
1080   60         L1080     RTS
