// Import the resume template
#import "templates/resume.typ": resume

// Sample resume data
#let data = (
  basics: (
    name: "John Doe",
    email: "john.doe@email.com",
    phone: "(555) 123-4567",
    location: "San Francisco, CA",
    summary: "Results-driven Senior Software Engineer with 8+ years of experience building scalable web applications and leading cross-functional teams. Passionate about clean code, system design, and mentoring junior developers. Proven track record of delivering high-impact projects that improve user experience and drive business growth.",
    profiles: (
      (
        network: "LinkedIn",
        url: "https://linkedin.com/in/johndoe"
      ),
      (
        network: "GitHub",
        url: "https://github.com/johndoe"
      ),
    ),
  ),

  education: (
    (
      institution: "University of California, Berkeley",
      degree: "Bachelor of Science",
      fieldOfStudy: "Computer Science",
      location: "Berkeley, CA",
      startDate: "2012-08",
      endDate: "2016-05",
      gpa: "3.7/4.0",
      highlights: (
        "Dean's List (6 semesters)",
        "Senior thesis on distributed systems optimization",
      ),
    ),
  ),

  work: (
    (
      company: "TechCorp Inc.",
      position: "Senior Software Engineer",
      location: "San Francisco, CA",
      startDate: "2021-03",
      endDate: "Present",
      highlights: (
        "Led refactoring of a monolithic architecture, reducing page load times by 40%",
        "Implemented microservices migration strategy that improved system reliability to 99.9% uptime",
        "Mentored team of 5 junior developers through code reviews and pair programming sessions",
        "Designed and deployed a real-time notification system serving 2M+ daily active users",
      ),
    ),
    (
      company: "StartupXYZ",
      position: "Software Engineer",
      location: "San Jose, CA",
      startDate: "2018-06",
      endDate: "2021-02",
      highlights: (
        "Built RESTful APIs and GraphQL endpoints handling 10K+ requests per second",
        "Developed automated testing workflow that reduced deployment errors from 85% to 5%",
        "Collaborated with product team to launch 3 major features that increased user retention by 25%",
        "Optimized database queries resulting in 60% reduction in API response times",
      ),
    ),
    (
      company: "WebDev Agency",
      position: "Junior Software Developer",
      location: "Oakland, CA",
      startDate: "2016-08",
      endDate: "2018-05",
      highlights: (
        "Developed responsive web applications for 15+ clients using React and Node.js",
        "Created reusable component library that reduced development time by 30%",
        "Participated in agile ceremonies and contributed to sprint planning",
      ),
    ),
  ),

  projects: (
    (
      name: "OpenSource Dashboard",
      description: "An analytics dashboard for tracking open-source project metrics",
      url: "https://github.com/johndoe/opensource-dashboard",
      keywords: ("React", "D3.js", "Node.js", "MongoDB"),
      highlights: (
        "Built with React, D3.js, and Node.js backend",
        "500+ GitHub stars and 50+ contributors",
      ),
    ),
  ),

  certifications: (
    (
      name: "AWS Certified Solutions Architect",
      issuer: "Amazon Web Services",
      date: "2023-01",
    ),
    (
      name: "Certified Kubernetes Administrator",
      issuer: "Cloud Native Computing Foundation",
      date: "2022-06",
    ),
  ),

  skills: (
    (
      name: "Programming Languages",
      keywords: ("JavaScript", "TypeScript", "Python", "Go", "Rust"),
    ),
    (
      name: "Frontend",
      keywords: ("React", "Vue.js", "Next.js", "Tailwind CSS", "Redux"),
    ),
    (
      name: "Backend",
      keywords: ("Node.js", "Express", "GraphQL", "REST APIs", "FastAPI"),
    ),
    (
      name: "Databases",
      keywords: ("PostgreSQL", "MongoDB", "Redis", "Elasticsearch"),
    ),
    (
      name: "DevOps & Cloud",
      keywords: ("Docker", "Kubernetes", "AWS", "GCP", "CI/CD", "Terraform"),
    ),
    (
      name: "Tools & Practices",
      keywords: ("Git", "Agile/Scrum", "Testing (Jest, Pytest)", "System Design"),
    ),
  ),
)

// Render the resume with the data
#resume(data)
