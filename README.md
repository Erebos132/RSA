# rsa-facharbeit

This Project is my (second) own Implementation of the RSA Encryption and Signature Algorithm. This time, I've imlemented it in rust, in the hopes of creating a faster and more efficient environment. 
Unfortunately, I have apparently failed to do so. At least for now. When performance might become multi-threaded, it might become a different deal. 


TODO-List: 
  - Graphs of: 
      - key generation: bitlength vs time
      - factorization: bitlength vs time
      - Encryption: bitlength vs time
      - Decryption: bitlength vs time
      - Padding-Hack: bitlength vs time
      - What operation takes how much time for key generation (circle diagram)
  - #Build Blocking of Messages
  - #Adding Padding
  - #Hashing Function
  - Multi-threaded generation of keys, encryption, and decryption
  - Improve Performance of Key Generation
  - Attacks: 
    - #technically low public exponent e (test and document)
        -> If m^e < n --> There is no modulous being taken, therefore, no encryption
    - #chosen cyphertext attack -> User error
    - Try to recover secrets by knowing encryption and cleartext-message (apparently not possible)
    - maybe coppersmith attack? -> Seems complex...
  - Add proper OAEP
