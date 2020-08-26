# HyperDarwin (HyperNEAT Darwin, Version Alpha)
HyperDarwin is a branching extension to my Darwin project that implements the Hypercube-based NeuroEvolution
of Augmenting Topologies Genetic Algorithm (HyperNEAT).
<br>
<br>
The plan is to test this out in a fully fledged game environment, which will be some time in the future.

## What is HyperNEAT?
HyperNEAT is kind of complex hence I recommend you to refer to the original research papers for NEAT, CPPNS
and ultimately HyperNEAT (Links down below). I will be explaining these briefly.
The statement "HyperNEAT is an extension of NEAT" is not entirely correct. HyperNEAT can be thought of
as an application of Compositional Pattern Producing Networks (CPPNs) which are in turn an extension of
the NEAT Genetic Algorithm.

### NEAT
For information about NEAT, check out my `Darwin` project.

### CPPN and CPPN-NEAT
Compositional Pattern Producing Networks (CPPNs) are simply Neural Networks that can be used to create
patterns. This is achieved by passing in the co-ordinates of a point in 2D space to the network, and the
output is the color value of the pixel at that position (Could be a single grayscale output or 3 RGB outputs, etc).
The co-ordinates are constrained between -1 and 1 (inclusive) on both axes. Multiple resolutions can be
represented as a grid of points in this 2x2 area.
<br>
Since the co-ordinates are always between -1 and 1, there is no upper bound on the resolution of the output
pattern that can be achieved.
<br>
CPPNs can theoretically approximate any pattern. This power can be attributed to the fact that different nodes
of CPPNs have different activation functions, chosen from a predefined universal set. The most common ones
can be divided into the following four categories L-
- Sigmoidal
- Periodic
- Linear
- Gaussian

Each type of function imparts a different property on the Pattern. For example, Gaussian functions allow
the output to be biased towards a mean, whereas Periodic functions allow for repetitions to occur, and so on.
<br>
<br>
CPPN-NEAT is basically NEAT with CPPNs as opposed to vanilla neural networks.


### HyperNEAT
Imagine two planes constrained within -1 and 1 along both axes (like in CPPNs) and having equal resolutions.
Now imagine all of the points in Plane 1 connected to all of the points in Plane 2, sort of like a 2D
Neural Network such that every node in Plane 1 is an input node and every node in Plane 2 is an output node.
<br>
<br>
And what about the weights between the connections?
<br>
<br>
Take a CPPN with 4 inputs and a single output. The input consists of the co-ordinate data of two points
(x1, y1), (x2, y2). The first point is in Plane 1, the second in Plane 2. The output of the CPPN is the
weight of the connection between these points.
<br>
Follow this process for all possible connections to get the fully connected 2D NN. Then pass any data
you want through this NN and get the output at the other end.
<br>
<br>
This is HyperNEAT. We Leverage the power of CPPNs to exploit the geometry of our Problem Space. I say
Problem Space instead of "2D Problem Space" cuz you can create n-dimensional Neural Networks with this
algorithm just by tweaking the inputs to the CPPN. This is a very surface-level explanation. Read the paper
to know more.

## References
- NEAT: [K. Stanley, R. Miikkulainen (2002) 'Evolving Neural Networks through Augmenting Topologies'](http://nn.cs.utexas.edu/downloads/papers/stanley.ec02.pdf)
- CPPN/CPPN-NEAT: [Kenneth O. Stanley (2007) 'Compositional Pattern Producing Networks: A Novel Abstraction of Development'](https://eplex.cs.ucf.edu/papers/stanley_gpem07.pdf)
- HyperNEAT: [K. Stanley, D. Ambrosio, J. Gauci (2009) 'A Hypercube-Based Indirect Encoding for Evolving Large-Scale Neural Networks'](http://axon.cs.byu.edu/~dan/778/papers/NeuroEvolution/stanley3**.pdf)
