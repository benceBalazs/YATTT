import { Component, OnInit } from '@angular/core';
import { DataApiService} from '../../services/data-api.service';
import { Chart, registerables } from 'chart.js';

Chart.register(...registerables);

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.css']
})
export class DashboardComponent implements OnInit {
  attendances: any[] = [];
  chart: any;

  constructor(private dataApiService: DataApiService) {}

  ngOnInit(): void {
    this.fetchAttendances();
  }

  fetchAttendances(): void {
    this.dataApiService.getAttendances().subscribe(
      (data) => {
        this.attendances = data;
        this.createChart();
      },
      (error) => {
        console.error('Error fetching attendances:', error);
      }
    );
  }

  createChart(): void {
    const lectureNames = this.attendances.map((a: any) => a.lecture_name);
    const durations = this.attendances.map((a: any) => a.duration);

    this.chart = new Chart('attendanceChart', {
      type: 'bar',
      data: {
        labels: lectureNames,
        datasets: [
          {
            label: 'Lecture Duration (hours)',
            data: durations,
            backgroundColor: 'rgba(75, 192, 192, 0.6)',
            borderColor: 'rgba(75, 192, 192, 1)',
            borderWidth: 1,
          },
        ],
      },
      options: {
        responsive: true,
        scales: {
          y: {
            beginAtZero: true,
            title: {
              display: true,
              text: 'Duration (hours)',
            },
          },
          x: {
            title: {
              display: true,
              text: 'Lecture Names',
            },
          },
        },
      },
    });
  }
}
