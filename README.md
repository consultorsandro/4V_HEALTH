# 🩺 4V_SAUDE

**4V_SAUDE** is a Rust-based command-line application designed to provide four essential health checks for individuals, following best practices in software architecture (SOLID principles).

---

## 🚀 Features

- **1️⃣ BMI (Body Mass Index) Calculation**
- **2️⃣ TMB (Basal Metabolic Rate) Calculation**
- **3️⃣ PGC (Body Fat Percentage) Calculation**
- **4️⃣ WHR (Waist-to-Hip Ratio) Calculation**

---

## 🛠️ Technologies

- [Rust](https://www.rust-lang.org/) – Safe, fast, and modern systems programming language
- Modular architecture for easy maintenance and scalability

---

## 📦 Project Structure
4V_SAUDE/ ├── src/ │ ├── main.rs │ ├── bmi.rs │ ├── metabolism.rs │ ├── body_fat.rs │ └── whr/ │ ├── mod.rs │ └── calculator.rs ├── Cargo.toml └── README.md
---

## 🧑‍💻 Usage

1. **Build the project:**
   ```sh
   cargo build

2. Run the application:
   cargo run

3. Follow the on-screen instructions to perform health checks.
_ _ _ 

📊 Health Checks
Check	Description	Input Required
🟦 BMI	Calculates Body Mass Index	Weight, Height
🔥 TMB	Calculates Basal Metabolic Rate	Weight, Height, Age, Gender
🧮 PGC	Calculates Body Fat Percentage	Weight, Height, Age, Gender
📏 WHR	Calculates Waist-to-Hip Ratio	Gender, Waist, Hip

🧩 SOLID Principles

* Single Responsibility: Each module handles a specific health check.
* Open/Closed: Easily extendable for new checks.
* Liskov Substitution: Interfaces and traits for calculators.
* Interface Segregation: Separate traits for each calculation.
* Dependency Inversion: High-level modules depend on abstractions.
_ _ _

📃 License
This project is licensed under the MIT License.
_ _ _

🤝 Contributing
Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.
_ _ _

👨‍⚕️ Authors
Developed by the Sandro Reis and IA prompts.