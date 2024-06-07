```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "validate"]

[problem]
points = 7
difficulty = 1
```

# 🖥️ Matrix Code
Given several lines of text with words written vertically across lines, extract the vertical words and output the **average length** of all words. 

## Input
Input is several lines in the following format.

```
            h       
            e     h 
      h     l     eh
      e     l     le
      l     o     ll
     hl           ol
     eo            o
     l     h   h    
     l     e h eh  h
    ho     l e le  e
    e     hl l ll  l
 h  l     eo lhol  l
 e  l     l  oe o  o
 lh o     l   l     
 le       o   l   h 
 ol           o   e 
  l               l 
  o               l 
                  o 
```
The average length for this input would be `5.0`.

## Output
Output a floating point number representing the average length of all of the extracted vertical words. 
