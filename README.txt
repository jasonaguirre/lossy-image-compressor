– Array2 from Noah D.
- Jason Aguirre and Eric Shaw
– TA Hours with Connor Gray and Isaac Chen
– Bitpacking correctly implemented, compress/decompress not implemented correctly
– Main.rs calls compress/decompress based on user input.
Compress trims image and calls process() on 2x2 blocks of pixels.
process calls convert_to_float which converts the block to floating-point representation.
Then convert_to_cs is called on the float_block to convert the block to color space,
the do_cdt() function is called on the color space block to perform discrete cosine transformation to obtain a,b,c,d,pb,pr values,
the a,b,c, and d values are converted to bits and then packed into a 64 vit word.
Decompress performs the inverse of the operations mentioned above.
– We spent about 2 - 3 hours analyzing the assignment
– We spent close to 30 hours solving the problems proposed to us in the project