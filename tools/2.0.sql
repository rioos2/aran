--
-- Database: `riodb` for 2.0
--

CREATE DATABASE IF NOT EXISTS riodb;

SET DATABASE = riodb;

--
-- Table structure for table `accounts`
-- created_at is the first timestamp
--- updated_at is the last timestamp
CREATE TABLE IF NOT EXISTS accounts (
  id SERIAL PRIMARY KEY,
  email STRING UNIQUE,
  first_name STRING,
  last_name STRING,
  phone STRING,
  api_key STRING,
  password STRING,
  states STRING,
  approval STRING,
  suspend STRING,
  registration_ip_address STRING,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP(),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP());

--
-- Table structure for table `projects`
-- foreign key account_id
CREATE TABLE IF NOT EXISTS projects(
  id SERIAL PRIMARY KEY,
  account INT NOT NULL REFERENCES accounts(id),
  name STRING,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP(),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP());

--
-- Table structure for table `plan_factory`
-- tag : [], artifacts: [], services: json
CREATE TABLE IF NOT EXISTS plan_factory (
  id SERIAL PRIMARY KEY,
  name STRING,
  description STRING,
  tags STRING,
  camp_version DECIMAL DEFAULT 1.2,
  origin STRING DEFAULT 'rioos:2.0',
  artifacts STRING,
  services STRING,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP(),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP());

  --
  -- Table structure for table `assembly_factory`
  -- items : []
CREATE TABLE IF NOT EXISTS assembly_factory (
  id SERIAL PRIMARY KEY,
  uri STRING,
  name STRING,
  description STRING,
  tags STRING,
  representation_skew STRING,
  total_items INT DEFAULT 0,
  Items_per_page INT DEFAULT 10,
  start_index INT DEFAULT 0,
  items STRING,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP(),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP());

  --
  -- Table structure for table `assembly`
  -- tags : [],component_collection : []
  -- operation_collection : [], sensor_collection : []
  -- metadata : json
CREATE TABLE  IF NOT EXISTS assembly (
  id SERIAL PRIMARY KEY,
  uri STRING,
  name STRING,
  description STRING,
  tags STRING,
  representation_skew STRING,
  external_management_resource STRING,
  component_collection STRING,
  plan STRING,
  operation_collection STRING,
  sensor_collection STRING,
  metadata STRING,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP(),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP());


--
-- Table structure for table `components`
-- tags : [], assembly_collection : []
-- artifact: json ? service: json ?
-- related_components_collection : [], operation_collection : []
-- sensor_collection : [], metadata : json
CREATE TABLE  IF NOT EXISTS components (
  id SERIAL PRIMARY KEY,
  uri STRING,
  name STRING,
  description STRING,
  tags STRING,
  representation_skew STRING,
  external_management_resource STRING,
  assembly_collection STRING,
  artifact STRING,
  service STRING,
  status STRING,
  related_components_collection STRING,
  operation_collection STRING,
  sensor_collection STRING,
  metadata STRING,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP(),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP());


--
-- Dumping data for table `plan_factory`
--

-- INSERT INTO plan_factory (`user_id`, `username`, `first_name`, `last_name`, `gender`, `password`, `status`) VALUES
-- (1, 'rogers63', 'david', 'john', 'Female', 'e6a33eee180b07e563d74fee8c2c66b8', 1);
