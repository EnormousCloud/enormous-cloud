   drop table if exists networks_list;
create table networks_list (
   name  text,
   coin  text, 
   test  bool,
   price decimal,
   link  text default '',
   active bool,
   PRIMARY KEY (name)
);

INSERT INTO networks_list (name, active, coin, test, price) VALUES 
   ('Bitcoin', true, 'bitcoin', false, 0),
   ('Ethereum', true, 'ethereum', false, 0),
   ('Ethereum 2', true, 'ethereum', false, 0),
   ('Rinkeby', false, 'ethereum', true, 0),
   ('Goerli', false, 'ethereum', false, 0);

-- properties of the network, hardcoded in database
drop table if exists networks_properties;
create table networks_properties (
   name        text,
   sortorder   int,
   label text,
   value text,
   link  text, 
   PRIMARY KEY (name, sortorder)
);

-- list of URLs to retrieve chain states
drop table if exists networks_chainstate;
create table networks_chainstate (
   url         text,   -- URL to retrieve state
   auth_header text default '',   -- Authorization header to retrieve state
   PRIMARY KEY (url)
);

INSERT INTO networks_chainstate (url, auth_header) VALUES
   ('https://enormous.cloud/chainstate', ''); -- todo: rinkeby, georli
