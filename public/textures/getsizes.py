


from PIL import Image 
import json


out = {}


for i in range(1, 2001):
    main = Image.open(rf"main/{i}.png")
    detail = Image.open(rf"detail/{i}.png")
    out[i] = {
        "main": main.size,
        "detail": detail.size,
    }
    main.close()
    detail.close()

json_obj = json.dumps(out, indent = 4) 
f = open("texture_sizes.json", "w")
f.write(json_obj)
f.close()
# print(out)