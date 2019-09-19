# 从零开始写 OS

## 前言

本系列文章记录了使用 Rust 编程语言编写一个小型操作系统的详细过程。每篇文章包含所需所有所需代码和相关知识点讲解。

- [代码](https://github.com/LearningOS/rcore_step_by_step/os)

- [文档](https://learningos.github.io/rcore_step_by_step_webdoc/)

## 如何使用

为了方便起见，建议使用 [docker](http://www.runoob.com/docker/docker-tutorial.html) ，可以省去配置环境的功夫。

在工作目录下创建 **Makefile** ：

```Makefile
docker:
	sudo docker run -it --mount type=bind,source=$(shell pwd)/..,destination=/mnt panqinglin/rust_riscv bash
```

进入 docker 后，执行 `cd mnt` ，即可看见工作目录，然后就可以开始写代码啦！

> 每一章或小节对应的源代码可以在 [GitHub](https://github.com/LearningOS/rcore_step_by_step) 的 commit 中找到， 且 commit log 与每一篇文章的主标题或副标题内容大致对应。

## reference

- https://github.com/rcore-os/rCore
- https://github.com/oscourse-tsinghua/rcore_plus/tree/lab8-rv32-tinyfs
- https://github.com/chyyuu/rcore_plus/tree/lab1-rv32-interrupt .. https://github.com/chyyuu/rcore_plus/tree/lab8-rv32-fs

## rCore Summer of Code 2019

- https://github.com/LearningOS/rcore_step_by_step/wiki/rCore-Summer-of-Code
