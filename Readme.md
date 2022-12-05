
# Port Scanner - Find the IPs You're Looking For

Pretty straight forward.  Binary is available at ~/bin/gscan, and accepts the following options:

Flag | Long Flag | Default | Description
------------- |------------- |------------- 
-t | --threads | 0 | Number of threads to utilize.  If 0, will consume all threads on the machine.
-p | --port | 80 | Comma delimited list of ports to check.
-f | --file | search.txt | Filename containing phrases to scan for, one phrase per-line.
-o | --output | gscan.log | Filename to write any results found.
-s | --seconds | 10 | Duration in seconds of timeout before moving onto the next IP.

## Usage

Just put your desired searche phrases, one per-line in search.txt and fire up the script with: ./gscan

Limit to 6 threads:
    ./gscan -t 6

Limit to 16 threads, search file at "mysearch" and output file at "my_finds.log" with:
    ./gscan -t 16 -f mysearch -o my_finds.log

## Contact

Any questions, issues, or desired modifcations e-mail me at matt@apexpl.io.  Enjoy.

 


