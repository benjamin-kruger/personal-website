+++
title = "Choosing the best shape with differential equations"
date = 2024-08-11
math = true
+++

Recently, I have felt the urge to brush up on some of the mathematics I used to surround myself with. This post is an attempt to scratch that itch and explore interesting problems in differential geometry.

Before diving into the deep end with definitions and theorems, I thought I would motivate this series with a driving philosophical question in Riemannian geometry: **What is the best shape?**

To refine this question, let's first understand the central objects of study in differential geometry: *manifolds*. These spaces are locally modelled on Euclidean space but can be very different globally. The prototypical example of a manifold is the sphere, $\mathbb{S}^n.$ A manifold is given geometry with an additional piece of data called a *Riemannian metric*, often denoted by $g$. These metrics give rise to familiar geometric notions such as distance, volume, and curvature. A manifold can be equipped with different metrics, and each choice results in different measures of distance, volume, curvature, etc. A natural reformulation of our question arises: given a manifold, is there a distinguished choice of metric?

For simply-connected Riemann surfaces (think two-dimensional surfaces with no holes), the Poincaré-Koebe uniformisation theorem provides a clear answer: every such surface admits a unique Riemannian metric of constant (Gauss) curvature, up to diffeomorphism. In fact, there are only three possibilities: the Euclidean plane, the sphere, and the hyperbolic plane. However, in higher dimensions, we encounter three distinct notions of curvature:

- Sectional curvature,
- Ricci curvature, and
- Scalar curvature.

In two dimensions, these notions are equivalent, but in higher dimensions, they differ.

The resolution of the [Yamabe problem](https://en.wikipedia.org/wiki/Yamabe_problem) tells us that every Riemannian metric on a compact manifold is conformally equivalent to one with constant scalar curvature. Moreover, any simply connected complete manifold with constant sectional curvature is isometric to Euclidean space, the sphere or hyperbolic space with their usual metrics. In this sense, seeking a metric with constant scalar curvature is too permissive, and seeking a metric with constant sectional curvature is too restrictive. Hence, in search of distinguished Riemannian metrics, we focus on those with constant Ricci curvature; such metrics are called *Einstein metrics* and are defined by the equation

$$
\mathrm{Ric}(g) = \lambda g, \quad \lambda\in \mathbb{R}.
$$

Not all manifolds admit Einstein metrics. An example in dimension three is $\mathbb{S}^2\times\mathbb{S}^1$. In this dimension, Ricci curvature coincides with sectional curvature, so a Riemannian manifold is Einstein if and only if it has constant sectional curvature (i.e., it is a discrete quotient of Euclidean space, the sphere or hyperbolic space). For manifolds that do not admit Einstein metrics, we must look elsewhere for a distinguished metric.

To search for other distinguished metrics, we reframe the above question as a dynamic problem: can a given Riemannian metric be improved, for instance, to have more symmetry or 'better' curvature properties? A popular approach to this problem is Hamilton's [Ricci flow](https://en.wikipedia.org/wiki/Ricci_flow). Given a Riemannian manifold $(M, g_0)$, a one-parameter family of Riemannian metrics $\{g(t)\}_{t \in [0, T)}$ is a *Ricci flow* with initial metric $g_0$ if it satisfies the initial value problem

$$
\frac{\partial}{\partial t} g(t) = -2 \mathrm{Ric}(g(t)), \quad g(0) = g_0.
$$

One can think of the Ricci flow as a non-linear analogue of the [heat equation](https://en.wikipedia.org/wiki/Heat_equation), which describes the dispersion of heat over time. The study of the Ricci flow became popular after the resolution of the famous [Poincaré conjecture](https://en.wikipedia.org/wiki/Poincar%C3%A9_conjecture) and [Thurston geometrization conjecture](https://en.wikipedia.org/wiki/Geometrization_conjecture) by Grigori Perelman in 2002 and 2003.

An excellent candidate for distinguished geometries is the self-similar solutions to the Ricci flow, i.e., those metrics that cannot be 'improved' by the Ricci flow any further. We call such Riemannian metrics *Ricci solitons*. All Einstein metrics are Ricci solitons and so, in some sense, they are the best shapes...

[← Back](/)
