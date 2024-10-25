                            * = $0001
0000   4A                   LSR A
0001   4D 50 DE             EOR $DE50
0004   00                   BRK

0010   01 00                ORA ($00,X)
0012   00                   BRK

00CA   29 00                AND #$00
00CC   00                   BRK

00DE   AD 00 00             LDA $0000
00E1   00                   BRK

00FE   1C                   ???                ;%00011100
00FF   00                   BRK

0700   FE F8 FD             INC $FDF8,X
0703   E8                   INX
0704   FF                   ???                ;%11111111
0705   FE E8 EE             INC $EEE8,X
0708   FF                   ???                ;%11111111
0709   E8                   INX
070A   F9 00 00             SBC $0000,Y
070D   00                   BRK

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
1031   AD 02 F0   L1031     LDA $F001
1034   F0 FB                BEQ L1031
1036   20 45 10             JSR L1045
1039   9D 10 DD             STA $DD10,X
103C   9D 71 20             STA $2071,X
103F   E8                   INX
1040   E0 1E                CPX #$1E
1042   D0 ED                BNE L1031
1044   60                   RTS
1045   AD 00 C0   L1045     LDA $C000
1048   CE 01 F0             DEC $F001
104B   60                   RTS
104C   BD 50 DD   L104C     LDA $DD50,X
104F   41 03                EOR ($03,X)
1051   DD 10 DD             CMP $DD10,X
1054   F0 F6                BEQ L104C
1056   60                   RTS
1057   A2 00      L1057     LDX #$00
1059   A9 00      L1059     LDA #$00
105B   9D 10 DD             STA $DD10,X
105E   9D 71 20             STA $2071,X
1061   E8                   INX
1062   E0 1E                CPX #$1E
1064   D0 F3                BNE L1059
1066   60                   RTS
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
1081   00                   BRK

2000   2A                   ROL A

2020   2A         L2020     ROL A
2021   20 20 20             JSR L2020

202A   20 52 45             JSR L4552
202D   41 50                EOR ($50,X)
202F   45 52                EOR $52
2031   20 47 41             JSR L4147
2034   54                   ???                ;%01010100 'T'
2035   45 20                EOR $20
2037   20 20 20             JSR L2020

203D   20 20 2A             JSR L2A20
2040   2A                   ROL A

2060   00                   BRK

2A20   00         L2A20     BRK
2A21   00                   BRK

4147   00         L4147     BRK
4148   00                   BRK

4552   00         L4552     BRK
4553   00                   BRK

7331   A5 CA      L7331     LDA $CA
7333   6A                   ROR A
7334   45 00                EOR $00
7336   48                   PHA
7337   A5 FE                LDA $FE
7339   A2 01                LDX #$01
733B   2A                   ROL A

733E   55 00                EOR $00,X
7340   48                   PHA
7341   A5 10                LDA $10
7343   A0 02                LDY #$02
7345   6A                   ROR A

7347   59 00 00             EOR $0000,Y
734A   48                   PHA
734B   68                   PLA
734C   A9 00                LDA #$00
734E   68                   PLA
734F   A9 00                LDA #$00
7351   68                   PLA
7352   A9 00                LDA #$00
7354   4C 31 73             JMP L7331
7357   00                   BRK

DD50   0F                   ???                ;%00001111
DD51   12                   ???                ;%00010010
DD52   1B                   ???                ;%00011011
DD53   00                   BRK
DD54   1C                   ???                ;%00011100
DD55   1E 03 0C             ASL $0C03,X
DD58   10 1D                BPL LDD77
DD5A   0A                   ASL A
DD5B   0B                   ???                ;%00001011
DD5C   06 1A                ASL $1A
DD5E   1F                   ???                ;%00011111
DD5F   17                   ???                ;%00010111
DD60   0E 16 05             ASL $0516
DD63   08                   PHP
DD64   13                   ???                ;%00010011
DD65   0D 04 01             ORA $0104
DD68   02                   ???                ;%00000010
DD69   19 15 09             ORA $0915,Y
DD6C   18                   CLC
DD6D   07                   ???                ;%00000111
DD6E   00                   BRK

DD77   00         LDD77     BRK
DD78   00                   BRK

DD80   FA                   ???                ;%11111010
DD81   C8                   INY
DD82   8D CC DF             STA $DFCC
DD85   C8                   INY
DD86   8D DE CC             STA $CCDE
DD89   CB                   ???                ;%11001011
DD8A   C8                   INY
DD8B   83                   ???                ;%10000011

DD8E   8D C2 DF             STA $DFC2
DD91   8D C0 CC             STA $CCC0
DD94   D4                   ???                ;%11010100
DD95   CF                   ???                ;%11001111
DD96   C8                   INY
DD97   8D C3 C2             STA $C2C3
DD9A   D9 83 8D             CMP $8D83,Y
DD9D   FA                   ???                ;%11111010
DD9E   C8                   INY
DD9F   C1 CE                CMP ($CE,X)
DDA1   C2                   ???                ;%11000010
DDA2   C0 C8                CPY #$C8
DDA4   8D D9 C5             STA $C5D9
DDA7   C8                   INY
DDA8   8D FF C8             STA $C8FF
DDAB   CC DD C8             CPY $C8DD
DDAE   DF                   ???                ;%11011111
DDAF   DE 83 83             DEC $8383,X
DDB2   83                   ???                ;%10000011
DDB3   00                   BRK

DE00   9A                   TXS
DE01   9B                   ???                ;%10011011
DE02   9C                   ???                ;%10011100
DE03   8B                   ???                ;%10001011
DE04   99 FE 8E             STA $8EFE,Y
DE07   BF                   ???                ;%10111111
DE08   AD AD A9             LDA $A9AD
DE0B   B1 AC                LDA ($AC),Y
DE0D   BA                   TSX
DE0E   E4 00                CPX $00
DE10   00                   BRK

FFFD   10 00                BPL LFFFF
FFFF   00         LFFFF     BRK
                            .END

;auto-generated symbols and labels
 L1000      $1000

