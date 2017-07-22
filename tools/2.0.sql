--
-- Table structure for table `projects`
-- foreign key account_id
CREATE TABLE IF NOT EXISTS projects(
  id serial PRIMARY KEY,
  account integer NOT NULL REFERENCES accounts(id),
  name text,
  updated_at timestamptz,
  created_at timestamptz DEFAULT now());

--
-- Table structure for table `plan_factory`
-- tag : [], artifacts: [], services: json
CREATE TABLE IF NOT EXISTS plan_factory (
  id serial PRIMARY KEY,
  name text,
  description text,
  tags text,
  camp_version DECIMAL DEFAULT 1.2,
  origin text DEFAULT 'rioos:2.0',
  artifacts text,
  services text,
  updated_at timestamptz,
  created_at timestamptz DEFAULT now());

  --
  -- Table structure for table `assembly_factory`
  -- items : []
CREATE TABLE IF NOT EXISTS assembly_factory (
  id serial PRIMARY KEY,
  uri text,
  name text,
  description text,
  tags text,
  representation_skew text,
  total_items INT DEFAULT 0,
  Items_per_page INT DEFAULT 10,
  start_index INT DEFAULT 0,
  items text,
  updated_at timestamptz,
  created_at timestamptz DEFAULT now());


--
-- Table structure for table `components`
-- tags : [], assembly_collection : []
-- artifact: json ? service: json ?
-- related_components_collection : [], operation_collection : []
-- sensor_collection : [], metadata : json
CREATE TABLE  IF NOT EXISTS components (
  id serial PRIMARY KEY,
  uri text,
  name text,
  description text,
  tags text[],
  representation_skew text,
  external_management_resource text,
  assembly_collection text[],
  artifact text,
  service text,
  status text,
  related_components_collection text[],
  operation_collection text[],
  sensor_collection text[],
  metadata text[],
  updated_at timestamptz,
  created_at timestamptz DEFAULT now());


--
-- Dumping data for table `plan_factory`
--

-- INSERT INTO plan_factory (`user_id`, `username`, `first_name`, `last_name`, `gender`, `password`, `status`) VALUES
-- (1, 'rogers63', 'david', 'john', 'Female', 'e6a33eee180b07e563d74fee8c2c66b8', 1);
