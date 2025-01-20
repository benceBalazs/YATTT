import { Component, OnInit } from '@angular/core';
import { DataApiService } from '../data-api.service';
import { Card } from '../project.model';
import {FormsModule} from '@angular/forms';
import {NgIf} from '@angular/common';

@Component({
  selector: 'app-cards',
  templateUrl: './cards.component.html',
  styleUrls: ['./cards.component.css'],
  imports: [
    FormsModule,
    NgIf
  ]
})
export class CardsComponent implements OnInit {
  cards: Card[] = [];
  cardForm: Card = { name: '', tag_id: '' };
  showModal = false;
  isEdit = false;

  constructor(private dataApiService: DataApiService) {}

  ngOnInit(): void {
    this.fetchCards();
  }

  fetchCards(): void {
    this.dataApiService.getCards().subscribe((cards) => {
      this.cards = cards;
    });
  }

  openAddModal(): void {
    this.isEdit = false;
    this.cardForm = { name: '', tag_id: this.generateTagId() };
    this.showModal = true;
  }

  openEditModal(card: Card): void {
    this.isEdit = true;
    this.cardForm = { ...card };
    this.showModal = true;
  }

  closeModal(): void {
    this.showModal = false;
  }

  saveCard(): void {
    if (this.isEdit) {
      this.dataApiService.updateCard(this.cardForm.tag_id, this.cardForm).subscribe(() => {
        this.fetchCards();
        this.closeModal();
      });
    } else {
      this.dataApiService.addCard(this.cardForm).subscribe(() => {
        this.fetchCards();
        this.closeModal();
      });
    }
  }

  deleteCard(tag_id: string): void {
    this.dataApiService.deleteCard(tag_id).subscribe(() => {
      this.fetchCards();
    });
  }

  generateTagId(): string {
    return "0";
  }
}
