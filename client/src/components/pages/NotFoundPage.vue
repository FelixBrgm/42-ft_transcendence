<template>
    <div>
      <canvas ref="canvas" @click="handleMouseClick"></canvas>
    </div>
  </template>
  
  <script>
  export default {
    mounted() {
      this.initialize();
      this.animate();
      this.loadSound();
    },
    methods: {
      initialize() {
        this.canvas = this.$refs.canvas;
        this.ctx = this.canvas.getContext("2d"); 
        this.image = new Image();
        this.image.src = require('@/assets/logo.gif');
        this.images = [];
  
        for (let i = 0; i < 200; i++) {
          this.images.push({ 
            x: Math.random() * this.canvas.width,
            y: Math.random() * this.canvas.height,
            dx: Math.random() * 2 + 1,
            dy: Math.random() * 2 + 1,
          });
        } 
  
        this.resizeCanvas();
        window.addEventListener('resize', this.resizeCanvas);
      },
      loadSound() {
        this.audio = new Audio(require('@/assets/honk.mp3')); 
      },
      resizeCanvas() {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
      },
      animate() {
        requestAnimationFrame(this.animate);
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.images.forEach((img) => {
          img.x += img.dx;
          img.y += img.dy;
  
          if (img.x < 0 || img.x > this.canvas.width) {
            img.dx = -img.dx;
          }
          if (img.y < 0 || img.y > this.canvas.height) {
            img.dy = -img.dy;
          }
  
          this.ctx.drawImage(this.image, img.x, img.y, 50, 50);
        });
      },
      playSound() {
        this.audio.play();
      },
      handleMouseClick(event) {
        const rect = this.canvas.getBoundingClientRect();
        const mouseX = event.clientX - rect.left;
        const mouseY = event.clientY - rect.top;
  
        // Check if mouse click is over the first image
        if (
          mouseX >= this.images[0].x &&
          mouseX <= this.images[0].x + 50 &&
          mouseY >= this.images[0].y &&
          mouseY <= this.images[0].y + 50
        ) {
          this.playSound();
        }
      }
    },
    beforeDestroy() {
      window.removeEventListener('resize', this.resizeCanvas);
    }
  };
  </script>
  
  <style scoped>
  canvas {
    border: 1px solid black;
    width: 100vw;
    height: 100vh;
  }
  </style>
  