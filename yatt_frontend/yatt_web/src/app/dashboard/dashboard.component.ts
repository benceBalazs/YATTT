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
  attendanceChart: any;
  cardUsageChart: any;

  constructor(private dataService: DataApiService) {}

  ngOnInit() {
    this.loadAttendanceData();
    this.loadCardUsageData();
  }

  loadAttendanceData() {
    this.dataService.getAttendances().subscribe((response: any) => {
      const datesMap = new Map<string, number>();

      response.forEach((record: any) => {
        const date = new Date(record.check_in_time).toLocaleDateString();
        datesMap.set(date, (datesMap.get(date) || 0) + 1);
      });

      const labels = Array.from(datesMap.keys());
      const values = Array.from(datesMap.values());

      this.createAttendanceChart(labels, values);
    });
  }

  createAttendanceChart(labels: string[], values: number[]) {
    if (this.attendanceChart) this.attendanceChart.destroy();

    this.attendanceChart = new Chart("attendanceChart", {
      type: 'bar',
      data: {
        labels: labels,
        datasets: [{
          label: 'Number of Attendances',
          data: values,
          backgroundColor: 'rgba(75, 192, 192, 0.6)',
          borderColor: 'rgba(75, 192, 192, 1)',
          borderWidth: 1
        }]
      },
      options: {
        responsive: true,
        scales: {
          y: {
            beginAtZero: true
          }
        }
      }
    });
  }

  loadCardUsageData() {
    this.dataService.getAttendances().subscribe((response: any) => {
      const cardMap = new Map<string, number>();

      response.forEach((card: any) => {
        cardMap.set(card.name, (cardMap.get(card.name) || 0) + 1);
      });

      const labels = Array.from(cardMap.keys());
      const values = Array.from(cardMap.values());

      this.createCardUsageChart(labels, values);
    });
  }

  createCardUsageChart(labels: string[], values: number[]) {
    if (this.cardUsageChart) this.cardUsageChart.destroy();

    this.cardUsageChart = new Chart("cardUsageChart", {
      type: 'pie',
      data: {
        labels: labels,
        datasets: [{
          label: 'Card Usage',
          data: values,
          backgroundColor: [
            'rgba(255, 99, 132, 0.6)',
            'rgba(54, 162, 235, 0.6)',
            'rgba(255, 206, 86, 0.6)',
            'rgba(75, 192, 192, 0.6)'
          ],
          borderColor: '#fff',
          borderWidth: 1
        }]
      },
      options: {
        responsive: true
      }
    });
  }
}
