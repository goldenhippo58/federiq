CREATE TABLE employees (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100),
    position VARCHAR(50),
    salary DECIMAL(10,2)
);

INSERT INTO employees (name, position, salary) VALUES
    ('Alice', 'Engineer', 80000.00),
    ('Bob', 'Manager', 95000.00),
    ('Charlie', 'HR', 60000.00);
