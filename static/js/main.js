document.addEventListener('DOMContentLoaded', function() {
    console.log('🦀 Rust Backend loaded successfully!');
    
    // API Status Check
    const checkStatusBtn = document.getElementById('checkStatus');
    const apiResult = document.getElementById('apiResult');
    
    if (checkStatusBtn && apiResult) {
        checkStatusBtn.addEventListener('click', async function() {
            try {
                // Show loading state
                apiResult.innerHTML = '<div class="loading">جاري التحقق...</div>';
                checkStatusBtn.disabled = true;
                
                // Fetch API status
                const response = await fetch('/api/status');
                const data = await response.json();
                
                // Display result
                apiResult.innerHTML = `
                    <div style="color: #28a745; font-weight: bold;">
                        ✅ ${data.message}
                        <br>
                        <small>Version: ${data.version}</small>
                    </div>
                `;
                
            } catch (error) {
                apiResult.innerHTML = `
                    <div style="color: #dc3545; font-weight: bold;">
                        ❌ خطأ في الاتصال: ${error.message}
                    </div>
                `;
            } finally {
                checkStatusBtn.disabled = false;
            }
        });
    }
    
    // Add smooth scrolling for any future navigation
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
    
    // Add card hover effects
    const cards = document.querySelectorAll('.card');
    cards.forEach(card => {
        card.addEventListener('mouseenter', function() {
            this.style.transform = 'translateY(-5px)';
        });
        
        card.addEventListener('mouseleave', function() {
            this.style.transform = 'translateY(0)';
        });
    });
});