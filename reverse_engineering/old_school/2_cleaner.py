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
