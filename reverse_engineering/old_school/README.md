# Old School

We are given the files `alpha` and `Reaper.bin`.

## Part 1 - The Emulator

From some cursory analysis, `alpha` seems to be a typical ELF-format binary.
Opening it up in https://dogbolt.org/, we see:

![](./_images/6502.png)

From some further analysis, `alpha` seems to be a `6502` assembly language emulator!
We pass it a file containing the initial state of memory to set up.

By the process of elimination, that file is probably `Reaper.bin`. Let's try it out:

![](./_images/reaper_bin_test.gif)

Nice! Seems like `Reaper.bin` probably contains some small program that requires us to provide a password,
which we'll give to access the key.
When we enter `30` characters into the password prompt, the screen is cleared.

## Part 2 - The ROM

With some blind faith that the `Reaper.bin` ROM file likely contains 6502 instructions,
we try to disassemble it with an online 6502 disassembler
(https://www.masswerk.at/ZZZZZZZ6502/disassembler.html).

That gives us a LONG stream of instructions that look like this:

[1_Reaper_dissassembled.asm](./1_Reaper_dissassembled.asm)

![](./_images/raw_dissassembly.png)

We see that most of the file consists of `0x00`, which corresponds to the 6502 `BRK` instruction.
Let's write a brief python program to squash duplicate instructions together:

[2_cleaner.py](./2_cleaner.py)
```py
with open("./1_Reaper_dissassembled.asm") as f:
    with open("./3_Reaper_clean.asm", "w") as f2:

        last_line = None
        gap_written = False

        for line in f.readlines():
            if last_line is None or len(line.split()) < 2 or len(last_line.split()) < 2 or last_line.split()[2:] != line.split()[2:]:
                f2.write(line)
                gap_written = False
            else:
                if not gap_written:
                    f2.write("\n")
                    gap_written = True

            last_line = line
```

This gives us an output that's much more readable:

[3_Reaper_clean.asm](./3_Reaper_clean.asm)

![](./_images/clean_dissassembly.png)

It looks like the disassembler didn't find much before `0x1000`, a lot of `0x00`s and invalid instructions
(signaled by `???` where the instruction should be), but starting at `0x1000`, it found plenty!
All instructions are valid and there are labels pointing from other places in the code
(indicated by `L1000`, etc in the third column).
This continues until offset `0x1081`, where we run into another long stream of zeros.

Signs seem to point to this at least being part of the code that gets executed, so let's start there.

## Part 3 - Subroutines

A subroutine in 6502 is called with a jump to subroutine instruction (`JSR`)
and returns by invoking an `RTS`.

Right away, we can see that there are segments of the assembly that start with a labeled line
(meaning they've been jumped to) and end with an `RTS`.

Breaking these chunks apart, it seems like our block of code consists of 6 subroutines and one "main" section of code
that calls out to many of them before terminating execution.

(Github unfortunately has fairly poor syntax highlighting for assembly, if that's distracting,
feel free to follow along in an editor of your choice!)

```nasm
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
```

This may look complex at this point, but by the end of this, we'll have determined what each line of code accomplishes.

To cover some basics of 6502 Assembly,
everything is based around CPU registers (primarily `A` and `X`),
which contain 1 byte at a time that the CPU can perform operations on.

For instance, subroutine 1 starts by "Loading into A" (`LDA`) the constant `0x0A`,
and then "Storing from A" (`STA`) into the memory location `0x2060`

```asm
; Subroutine 1
101C   A9 0A      L101C     LDA #$0A    ; Store the value 0x0A to $2060
101E   8D 60 20             STA $2060
...
```

Here is a quick reference table (homemade) for the instructions this program uses:

| Instruction | Mnemonic                | Notes                                                                                                                              |
|-------------|-------------------------|------------------------------------------------------------------------------------------------------------------------------------|
| LDA         | Load into A             |                                                                                                                                    |
| STA         | Store from A            |                                                                                                                                    |
| LDX         | Load into X             |                                                                                                                                    |
| STX         | Store from X            |                                                                                                                                    |
|             | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| EOR         | Exclusive OR            | XOR the value in the `A` register against some value                                                                               |
| INX         | Increment X             | Add `1` to the `X` register                                                                                                        |
| DEC         | Decrement               | Decrement a value in memory.                                                                                                       |
|             | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| CMP         | Compare against A       | Compare the value in the `A` register to some value, saving whether it was equal, less than, or greater than to the **CPU flags**  |
| CPX         | Compare against X       | Compare the value in the `X` register to some value, saving whether it was equal, less than, or greater than to the **CPU flags**  |
| BNE         | Branch if not equal     | Jump execution to a different location in the code if the last comparison **did not** set the `equals` cpu flag.                   |
| BEQ         | Branch if equal         | Jump execution to a different location in the code if the last comparison set the `equals` cpu flag.                               |
| JMP         | Jump                    | Jump execution to a different location in the code unconditionally, and without expecting the code to return.                      |
|             | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| JSR         | Jump to Subroutine      | Jump execution to a subroutine at a given offset                                                                                   |
| RTS         | Return from Subroutine  | Jump execution back to the code that called this subroutine.                                                                       |

With this, we should now be able to analyze our subroutines instruction-by-instruction.
Let's dive into deciphering the meaning of each!

### Subroutine 1

```asm
; Subroutine 1
101C   A9 0A      L101C     LDA #$0A    ; Store the value 0x0A to $2060
101E   8D 60 20             STA $2060
1021   A2 00                LDX #$00    ; Set X to 0
1023   BD 00 DE   L1023     LDA $DE00,X ; Load the value at memory address $DE00 + X to A
1026   45 03                EOR $03     ; XOR A with the value stored at address 0x03
1028   9D 61 20             STA $2061,X ; Store A to the memory address $2061 + X
102B   E8                   INX         ; Increment X
102C   E0 0F                CPX #$0F    ; If X is not equal to 0x0F, branch back to offset L1023
102E   D0 F3                BNE L1023
1030   60                   RTS         ; Exit the subroutine
```

As a note, you'll notice that there are a few instructions which take an address (like `LDA` and `STA`) which can be given an **indexed address**.
For example, at offset `0x1023`, the `LDA` instruction takes the address `$DE00,X`.
This just means that instead of loading from the address `$DE00`, we first add the current value of `X`.
So for `X=0`, we'd load from `$DE00`, but for `X=5`, we'd load from `$DE05`

With these translations, we can start to get an idea of what the program is doing.
First, it stores one byte to `$2060`.
Then in a loop, it takes one byte from memory, XORs it with the value stored at `0x03`, and stores it back to a different point in memory.

Ultimately, it takes the block of memory from `$DE00` to `$DE0F`, and writes it back to the location `$2061` (XORed by some value).

A quick python script to perform this operation:

[5_extract_constants.py](./5_extract_constants.py)

```py
with open("Reaper.bin", "rb") as f:
    reaper_bin = list(f.read())

    print("Memory from $DE00 -> $DEOF XORed with the value at address $03:")
    xor_constant = reaper_bin[0x03]
    print(bytes([x ^ xor_constant for x in reaper_bin[0xDE00:0xDE0F]]))
```

![](./_images/de00-de0f.png)

This seems to be the prompt we're given when the program starts!
This likely means that the address it is written, `$2061`, is some sort of output/screen buffer. That's good to keep in mind.

We'll call this function `write_prompt`

### Subroutine 3

```asm
; Subroutine 3
1045   AD 00 C0   L1045     LDA $C000   ; Load the address $C000 to A
1048   CE 01 F0             DEC $F001   ; Decrement the value in memory at $F001 by 1
104B   60                   RTS         ; Exit the subroutine
```

This subroutine is simple, but obscure.
We'll hope for now that we can better analyze it in context.
We'll call this function `load_c000 -> A`

### Subroutine 2

```asm
; Subroutine 2
1031   AD 02 F0   L1031     LDA $F001   ; Load the value at address $F001 to A
1034   F0 FB                BEQ L1031   ; Loop if the value read was 0
1036   20 45 10             JSR L1045   ; Call the "load_c000 -> A" subroutine
1039   9D 10 DD             STA $DD10,X ; Store A to [$DD10 + X]
103C   9D 71 20             STA $2071,X ; Store A to [$2071 + X]
103F   E8                   INX         ; Increment X by 1
1040   E0 1E                CPX #$1E    ; Loop if X is not equal to 0x1E
1042   D0 ED                BNE L1031
1044   60                   RTS         ; Exit the subroutine
```

This subroutine is another loop indexed with `X`.

The first two instructions are a smaller, nested loop, jumping back and forth until $F001 is 0.
Nothing here changes that address, so this must be some a special memory location that updates in response to something outside the program.

Once the change is detected, the `load_c000 -> A` function is called.
`c000` is another address that is never modified, maybe it is also a special location that updates for user input?

The value of A is then stored to both a section of memory `$DD10` and our output/screen buffer from before!

Putting all of this together, it seems like this function is mostly likely an "input collection" routine,
reading characters of user input and writing them back to the screen and storing them in a location `$DD10`.
This perfectly explains why we need to type `0x1e` (30) characters for the program to advance!

We'll call this subroutine `input`, and we'll call the block of data it writes (`$DD10`) the "input buffer".

### Subroutine 4

```asm
; Subroutine 4
104C   BD 50 DD   L104C     LDA $DD50,X ; Load the value at address [$DD50 + X] to A
104F   41 03                EOR ($03,X) ; XOR the value in A **indirectly** with the value contained at the address contained at [$03 + X]
1051   DD 10 DD             CMP $DD10,X ; Loop back to the start if A equals the value at [$DD10 + X]
1054   F0 F6                BEQ L104C
1056   60                   RTS         ; Exit the subroutine
```

Here we see our first case of **indirect** indexing.
Line `104F` contains the instruction `EOR ($03,X)`.
The parenthesis here mean that, instead of treating the address `[$03 + X]` as data,
we treat it as holding **another address**.
The location pointed to by that address is what actually contains the data.

So, this subroutine seems to compare the data from some section of memory (`$DD50`) XORed
with some value we've looked up indirectly from the binary.
Seems like this could check the validity of our password!
We'll call this subroutine `check_input`.

### Subroutine 5

```asm
; Subroutine 5
1057   A2 00      L1057     LDX #$00    ; Load 0 to X
1059   A9 00      L1059     LDA #$00    ; Load 0 to a
105B   9D 10 DD             STA $DD10,X ; Store A to the address [$DD10 + X]
105E   9D 71 20             STA $2071,X ; Store A to the address [$2071 + X]
1061   E8                   INX         ; Increment X by 1
1062   E0 1E                CPX #$1E    ; Loop back to the start if X != 0x1E
1064   D0 F3                BNE L1059   
1066   60                   RTS         ; Exit the subroutine
```

This subroutine contains another loop over X.
This time, it seems like it stores `0x1E` (30) bytes of `0x00` to two blocks of memory: `$DD10 to $DD2e` and `$2071 to $208f`.
These are the user input and output/screen buffer respectively.
We'll call this subroutine `clear_memory`.

### Subroutine 6

```asm
; Subroutine 6
1067   A2 00      L1067     LDX #$00    ; Load 0 to X
1069   BD 80 DD   L1069     LDA $DD80,X ; Load the value at [$DD80 + X] to A
106C   F0 12                BEQ L1080   ; If A = 0, then break the loop
106E   85 25                STA $25     ; Store A to the address $25
1070   BD 00 07             LDA $0700,X ; Load the value at [$0700 + X] to A
1073   41 03                EOR ($03,X) ; XOR the value in A **indirectly** with the value contained at the address contained at [$03 + X]
1075   A5 25                LDA $25     ; Load the value previously stored to address $25 to A
1077   45 DE                EOR $DE     ; XOR the value in A with the value at the address $DE
1079   9D 71 20             STA $2071,X ; Store A to the address [$2071 + X]
107C   E8                   INX         ; Increment X by 1
107D   4C 69 10             JMP L1069   ; Loop back to the start
1080   60         L1080     RTS         ; Exit the subroutine
```

This is another loop, but instead of looping to a particular value, it seems like
it loops until a 0 is read from memory.

Memory is read from `$DD80` plus some offset,
but then it is stored, 
before an additional value is loaded and XORed (lines `1070` and `1073`).
However, you'll notice that those values are never used, and they're immediately overwritten by reloading the data we
originally retrieved.
These intermediate steps are just a red herring.
Without this, we can rewrite this function as:

```asm
; Subroutine 6
1067   A2 00      L1067     LDX #$00    ; Load 0 to X
1069   BD 80 DD   L1069     LDA $DD80,X ; Load the value at [$DD80 + X] to A
106C   F0 12                BEQ L1080   ; If A = 0, then break the loop
1077   45 DE                EOR $DE     ; XOR the value in A with the value at the address $DE
1079   9D 71 20             STA $2071,X ; Store A to the address [$2071 + X]
107C   E8                   INX         ; Increment X by 1
107D   4C 69 10             JMP L1069   ; Loop back to the start
1080   60         L1080     RTS         ; Exit the subroutine
```

This shows much more clearly that the block of memory read from `$DD80` is XORed by some constant
from memory, before being stored back to `$2071`.

To figure out the data that's being read, let's write a python script to repeat this process by editing our earlier script:

[5_extract_constants.py](./5_extract_constants.py)

```py
with open("Reaper.bin", "rb") as f:
    ...

    print("Memory from $DD80 XORed with the value at address $DE:")
    xor_constant = reaper_bin[0xDE]

    payload = []
    i = 0
    while (c := reaper_bin[0xDD80 + i]) != 0x00:
        payload.append(c ^ xor_constant)
        i += 1

    print(bytes(payload))
```

![](./_images/dd80_xor.png)

Now we're getting somewhere! This subroutine results in this ominous message being written to the output buffer.
Let's call this function `write_message`

Now that we understand each of the subroutines, let's put them together with the main function.

## Part 5 - Program Flow

Substituting in all of our function calls, we have our main function:

```asm
; "Main" code
1000   20 57 10   L1000     JSR L1057   ; clear_memory
1003   20 1C 10             JSR L101C   ; write_prompt
1006   A2 00                LDX #$00    ; Write 0 to $F001
1008   8E 01 F0             STX $F001
100B   20 31 10             JSR L1031   ; input
100E   A2 00                LDX #$00    ; Set X to 0
1010   20 4C 10             JSR L104C   ; check_input
1013   D0 EB                BNE L1000   ; Loop back if the check_input call ended with a byte being not equal to the expected password
1015   20 67 10   L1015     JSR L1067   ; write_message
1018   4C 15 10             JMP L1015   ; Loop and write the output message again
101B   00                   BRK
```

Putting everything together, it seems like our program is waiting for input,
writing it to `$DD10`,
checking it with check_input,
and then calling `write_message` if the check succeeds.

We could try to decipher the password check_input is expecting,
but this is actually much harder (impossible?) to do in comparison to extracting the message right from `write_message`.

> As an aside, I have put quite a bit of effort into extracting the password,
> and with how the binary and emulator is configured, I don't think it's possible.
> The password (checked with a dynamic emulator at the comparison at offset `$1051`) is expected to be:
> ```py 
> b'\xa2XQJVTIFZW@A\x06WU]D\\OBYGNKHS_CRM' 
> ```
> There is one unprintable and one invalid ASCII character here, and the emulator specifically throws away non-printable
> characters, with the following segment of the decompiled `main` function:
> ```c
> int32_t rax_46 = wgetch(stdscr);
> if ((rax_46 - 0x20) <= 0x5e)
> {
>     void* rdx_4;
>     rdx_4 = rax->ident.abi_version;
>     *rdx_4[1] = rax->ident.pad[0];
>     *rdx_4[2] = rax->ident.pad[1];
>     *rdx_4[3] = rax->ident.pad[2];
>     *rdx_4[4] = rax->ident.pad[3];
>     *rdx_4[5] = rax->ident.pad[4];
>     *rdx_4[6] = rax->ident.pad[5];
>     *rdx_4[7] = rax->ident.pad[6];
>     *(rdx_4 + 0xf001) = 1;
>     void* rdx_5;
>     rdx_5 = rax->ident.abi_version;
>     *rdx_5[1] = rax->ident.pad[0];
>     *rdx_5[2] = rax->ident.pad[1];
>     *rdx_5[3] = rax->ident.pad[2];
>     *rdx_5[4] = rax->ident.pad[3];
>     *rdx_5[5] = rax->ident.pad[4];
>     *rdx_5[6] = rax->ident.pad[5];
>     *rdx_5[7] = rax->ident.pad[6];
>     *(rdx_5 + 0xc000) = rax_46;
> }
> ```
> You can see `$f001` being set to `1`, and `$c000` being set to the input character,
> just as we guessed from the `input` routine!
> But you can also see that `rax_46` must be within the range 0x20-0x7e, so there is no way to input the required `0xa2` or `0x06` bytes.
> 
> If anyone figures out if there is a password that works here, I would be very happy to know! But for the time being,
> we'll extract the message from the binary and assume that the password is left incomplete.

And as it turns out, the ominous message was the key we needed! Inputting `We are safe... or maybe not. Welcome the Reapers...` to the netcat terminal gives us the key.
Pwned/QED
