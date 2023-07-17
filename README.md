# A bookmark with logarithmic timescales written in Rust

This is a quick little toy-project. Certain Scifi-books measure time not in *minutes*,*hours*,*days*,*weeks* and so on, but instead use *seconds*, *kiloseconds*, *megaseconds* and so on. 
I wanted to create a logarithmic scale that maps between these quantities. 

A few examples:
| duration in seconds | duration in natural units |
|---------------------|---------------------------|
| 1 second | 1 second |
| 60 seconds | 1 minute |
| 1 kilosecond (1000 seconds) | 16 minutes 40 seconds |
| 1 megasecond (1000_000 seconds) | 11 days 13 hours 46 minutes 40 seconds |
| 1 gigasecond (1000_000_000 seconds) | 31 years 259 days 1 hour 46 minutes 40 seconds |

Feel free to print this out and use it when you read books that use kiloseconds, megaseconds or gigaseconds :-)