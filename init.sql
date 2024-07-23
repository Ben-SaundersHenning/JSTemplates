--
-- PostgreSQL modified database dump
--

-- Dumped from database version 13.13 (Debian 13.13-0+deb11u1)
-- Dumped by pg_dump version 13.13 (Debian 13.13-0+deb11u1)

CREATE USER jstg
with password 'password';

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: assessment_types; Type: TABLE; Schema: public; Owner: jstg
--

CREATE TABLE public.assessment_types (
    id integer NOT NULL,
    name text,
    common_name text
);


ALTER TABLE public.assessment_types OWNER TO jstg;

--
-- Name: assessment_types_id_seq; Type: SEQUENCE; Schema: public; Owner: jstg
--

CREATE SEQUENCE public.assessment_types_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.assessment_types_id_seq OWNER TO jstg;

--
-- Name: assessment_types_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: jstg
--

ALTER SEQUENCE public.assessment_types_id_seq OWNED BY public.assessment_types.id;

CREATE TYPE public.gender AS ENUM ('male', 'female', 'other');

--
-- Name: assessors; Type: TABLE; Schema: public; Owner: jstg
--

CREATE TABLE public.assessors (
    registration_id character(8) NOT NULL,
    first_name text NOT NULL,
    last_name text NOT NULL,
    gender public.gender NOT NULL,
    email text NOT NULL,
    qualifications_paragraph text NOT NULL
);


ALTER TABLE public.assessors OWNER TO jstg;

--
-- Name: documents; Type: TABLE; Schema: public; Owner: jstg
--

CREATE TABLE public.documents (
    id integer NOT NULL,
    name text NOT NULL,
    user_friendly_name text NOT NULL,
    common_name text NOT NULL,
    file text NOT NULL
);


ALTER TABLE public.documents OWNER TO jstg;

--
-- Name: documents_id_seq; Type: SEQUENCE; Schema: public; Owner: jstg
--

CREATE SEQUENCE public.documents_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.documents_id_seq OWNER TO jstg;

--
-- Name: documents_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: jstg
--

ALTER SEQUENCE public.documents_id_seq OWNED BY public.documents.id;


--
-- Name: referral_companies; Type: TABLE; Schema: public; Owner: jstg
--

CREATE TABLE public.referral_companies (
    id integer NOT NULL,
    name text NOT NULL,
    common_name text NOT NULL,
    address text NOT NULL,
    city text NOT NULL,
    province text NOT NULL,
    postal_code text NOT NULL,
    country text DEFAULT 'Canada'::text NOT NULL,
    phone text NOT NULL,
    fax text NOT NULL,
    email text NOT NULL
);


ALTER TABLE public.referral_companies OWNER TO jstg;

--
-- Name: referral_companies_id_seq; Type: SEQUENCE; Schema: public; Owner: jstg
--

CREATE SEQUENCE public.referral_companies_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.referral_companies_id_seq OWNER TO jstg;

--
-- Name: referral_companies_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: jstg
--

ALTER SEQUENCE public.referral_companies_id_seq OWNED BY public.referral_companies.id;


--
-- Name: assessment_types id; Type: DEFAULT; Schema: public; Owner: jstg
--

ALTER TABLE ONLY public.assessment_types ALTER COLUMN id SET DEFAULT nextval('public.assessment_types_id_seq'::regclass);


--
-- Name: documents id; Type: DEFAULT; Schema: public; Owner: jstg
--

ALTER TABLE ONLY public.documents ALTER COLUMN id SET DEFAULT nextval('public.documents_id_seq'::regclass);


--
-- Name: referral_companies id; Type: DEFAULT; Schema: public; Owner: jstg
--

ALTER TABLE ONLY public.referral_companies ALTER COLUMN id SET DEFAULT nextval('public.referral_companies_id_seq'::regclass);

--
-- Name: assessment_types assessment_types_pkey; Type: CONSTRAINT; Schema: public; Owner: jstg
--

ALTER TABLE ONLY public.assessment_types
    ADD CONSTRAINT assessment_types_pkey PRIMARY KEY (id);


--
-- Name: assessors assessors_pkey; Type: CONSTRAINT; Schema: public; Owner: jstg
--

ALTER TABLE ONLY public.assessors
    ADD CONSTRAINT assessors_pkey PRIMARY KEY (registration_id);


--
-- Name: documents documents_pkey; Type: CONSTRAINT; Schema: public; Owner: jstg
--

ALTER TABLE ONLY public.documents
    ADD CONSTRAINT documents_pkey PRIMARY KEY (id);


--
-- Name: referral_companies referral_companies_pkey; Type: CONSTRAINT; Schema: public; Owner: jstg
--

ALTER TABLE ONLY public.referral_companies
    ADD CONSTRAINT referral_companies_pkey PRIMARY KEY (id);

--
-- documents test data
--

insert into public.documents (name, user_friendly_name, common_name, file) values ('Test Document 1', 'Doc1', 'TD1', 'TD1.docx');
insert into public.documents (name, user_friendly_name, common_name, file) values ('Test Document 2', 'Doc2', 'TD2', 'TD2.docx');
insert into public.documents (name, user_friendly_name, common_name, file) values ('Test Document 3', 'Doc3', 'TD3', 'TD3.docx');
insert into public.documents (name, user_friendly_name, common_name, file) values ('Test Document 4', 'Doc4', 'TD4', 'TD4.docx');
insert into public.documents (name, user_friendly_name, common_name, file) values ('Test Document 5', 'Doc5', 'TD5', 'TD5.docx');

--
-- referral_companies test data
--

insert into public.referral_companies (name, common_name, address, city, province, country, postal_code, phone, fax, email) values
('Nalgene', 'Nalgene', '123 Water Street, Suite 1500', 'Toronto', 'Ontario', 'Canada', 'M1M 1M1', '999-999-9999', '999-999-9999', 'info@nalgene.com');

insert into public.referral_companies (name, common_name, address, city, province, country, postal_code, phone, fax, email) values
('Excel', 'Execel', '123 Gum Street', 'Toronto', 'Ontario', 'Canada', 'M9M 9M9', '111-111-1111', '111-111-1111', 'info@excel.com');

--
-- assessors test data
--

insert into public.assessors (registration_id, first_name, last_name, gender, email, qualifications_paragraph) values 
('G1234569', 'Frodo', 'Baggins', 'male', 'frodo@lotr.com', 'Ring Bearer');

insert into public.assessors (registration_id, first_name, last_name, gender, email, qualifications_paragraph) values 
('G1234559', 'Bilbo', 'Baggins', 'male', 'bilbo@lotr.com', 'Ring Bearer');

insert into public.assessors (registration_id, first_name, last_name, gender, email, qualifications_paragraph) values 
('G6543219', 'Tom', 'Bombadil', 'male', 'tom@lotr.com', 'Yellow Boots');

insert into public.assessors (registration_id, first_name, last_name, gender, email, qualifications_paragraph) values 
('G1597539', 'Goldberry', 'River-daughter', 'female', 'goldberry@lotr.com', 'lilies');

insert into public.assessors (registration_id, first_name, last_name, gender, email, qualifications_paragraph) values 
('G1597539', 'Treebeard', 'Fangorn', 'other', 'treebeard@lotr.com', 'Fangorn Forest');

--
-- assessment_types test data
--
insert into public.assessment_types (name, common_name) values ('Assessment type 1', 'at1');
insert into public.assessment_types (name, common_name) values ('Assessment type 2', 'at2');
insert into public.assessment_types (name, common_name) values ('Assessment type 3', 'at3');
