#[doc = "Register `MAPR` reader"]
pub type R = crate::R<MAPR_SPEC>;
#[doc = "Register `MAPR` writer"]
pub type W = crate::W<MAPR_SPEC>;
#[doc = "Field `SPI1_REMAP` reader - SPI1 remapping"]
pub type SPI1_REMAP_R = crate::BitReader;
#[doc = "Field `SPI1_REMAP` writer - SPI1 remapping"]
pub type SPI1_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_REMAP` reader - I2C1 remapping"]
pub type I2C1_REMAP_R = crate::BitReader;
#[doc = "Field `I2C1_REMAP` writer - I2C1 remapping"]
pub type I2C1_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1_REMAP` reader - USART1 remapping"]
pub type USART1_REMAP_R = crate::BitReader;
#[doc = "Field `USART1_REMAP` writer - USART1 remapping"]
pub type USART1_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2_REMAP` reader - USART2 remapping"]
pub type USART2_REMAP_R = crate::BitReader;
#[doc = "Field `USART2_REMAP` writer - USART2 remapping"]
pub type USART2_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3_REMAP` reader - USART3 remapping"]
pub type USART3_REMAP_R = crate::FieldReader;
#[doc = "Field `USART3_REMAP` writer - USART3 remapping"]
pub type USART3_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM1_REMAP` reader - TIM1 remapping"]
pub type TIM1_REMAP_R = crate::FieldReader;
#[doc = "Field `TIM1_REMAP` writer - TIM1 remapping"]
pub type TIM1_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM2_REMAP` reader - TIM2 remapping"]
pub type TIM2_REMAP_R = crate::FieldReader;
#[doc = "Field `TIM2_REMAP` writer - TIM2 remapping"]
pub type TIM2_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM3_REMAP` reader - TIM3 remapping"]
pub type TIM3_REMAP_R = crate::FieldReader;
#[doc = "Field `TIM3_REMAP` writer - TIM3 remapping"]
pub type TIM3_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM4_REMAP` reader - TIM4 remapping"]
pub type TIM4_REMAP_R = crate::BitReader;
#[doc = "Field `TIM4_REMAP` writer - TIM4 remapping"]
pub type TIM4_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN_REMAP` reader - CAN1 remapping"]
pub type CAN_REMAP_R = crate::FieldReader;
#[doc = "Field `CAN_REMAP` writer - CAN1 remapping"]
pub type CAN_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD01_REMAP` reader - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub type PD01_REMAP_R = crate::BitReader;
#[doc = "Field `PD01_REMAP` writer - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub type PD01_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5CH4_IREMAP` reader - Set and cleared by software"]
pub type TIM5CH4_IREMAP_R = crate::BitReader;
#[doc = "Field `TIM5CH4_IREMAP` writer - Set and cleared by software"]
pub type TIM5CH4_IREMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETRGINJ_REMAP` reader - ADC 1 External trigger injected conversion remapping"]
pub type ADC1_ETRGINJ_REMAP_R = crate::BitReader;
#[doc = "Field `ADC1_ETRGINJ_REMAP` writer - ADC 1 External trigger injected conversion remapping"]
pub type ADC1_ETRGINJ_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETRGREG_REMAP` reader - ADC 1 external trigger regular conversion remapping"]
pub type ADC1_ETRGREG_REMAP_R = crate::BitReader;
#[doc = "Field `ADC1_ETRGREG_REMAP` writer - ADC 1 external trigger regular conversion remapping"]
pub type ADC1_ETRGREG_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETRGINJ_REMAP` reader - ADC 2 external trigger injected conversion remapping"]
pub type ADC2_ETRGINJ_REMAP_R = crate::BitReader;
#[doc = "Field `ADC2_ETRGINJ_REMAP` writer - ADC 2 external trigger injected conversion remapping"]
pub type ADC2_ETRGINJ_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETRGREG_REMAP` reader - ADC 2 external trigger regular conversion remapping"]
pub type ADC2_ETRGREG_REMAP_R = crate::BitReader;
#[doc = "Field `ADC2_ETRGREG_REMAP` writer - ADC 2 external trigger regular conversion remapping"]
pub type ADC2_ETRGREG_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWJ_CFG` writer - Serial wire JTAG configuration"]
pub type SWJ_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    pub fn spi1_remap(&self) -> SPI1_REMAP_R {
        SPI1_REMAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    pub fn i2c1_remap(&self) -> I2C1_REMAP_R {
        I2C1_REMAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1_remap(&self) -> USART1_REMAP_R {
        USART1_REMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2_remap(&self) -> USART2_REMAP_R {
        USART2_REMAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    pub fn usart3_remap(&self) -> USART3_REMAP_R {
        USART3_REMAP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    pub fn tim1_remap(&self) -> TIM1_REMAP_R {
        TIM1_REMAP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    pub fn tim2_remap(&self) -> TIM2_REMAP_R {
        TIM2_REMAP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    pub fn tim3_remap(&self) -> TIM3_REMAP_R {
        TIM3_REMAP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline(always)]
    pub fn tim4_remap(&self) -> TIM4_REMAP_R {
        TIM4_REMAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline(always)]
    pub fn can_remap(&self) -> CAN_REMAP_R {
        CAN_REMAP_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01_remap(&self) -> PD01_REMAP_R {
        PD01_REMAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set and cleared by software"]
    #[inline(always)]
    pub fn tim5ch4_iremap(&self) -> TIM5CH4_IREMAP_R {
        TIM5CH4_IREMAP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC 1 External trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrginj_remap(&self) -> ADC1_ETRGINJ_REMAP_R {
        ADC1_ETRGINJ_REMAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC 1 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrgreg_remap(&self) -> ADC1_ETRGREG_REMAP_R {
        ADC1_ETRGREG_REMAP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC 2 external trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc2_etrginj_remap(&self) -> ADC2_ETRGINJ_REMAP_R {
        ADC2_ETRGINJ_REMAP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC 2 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc2_etrgreg_remap(&self) -> ADC2_ETRGREG_REMAP_R {
        ADC2_ETRGREG_REMAP_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_remap(&mut self) -> SPI1_REMAP_W<MAPR_SPEC> {
        SPI1_REMAP_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_remap(&mut self) -> I2C1_REMAP_W<MAPR_SPEC> {
        I2C1_REMAP_W::new(self, 1)
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_remap(&mut self) -> USART1_REMAP_W<MAPR_SPEC> {
        USART1_REMAP_W::new(self, 2)
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart2_remap(&mut self) -> USART2_REMAP_W<MAPR_SPEC> {
        USART2_REMAP_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_remap(&mut self) -> USART3_REMAP_W<MAPR_SPEC> {
        USART3_REMAP_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim1_remap(&mut self) -> TIM1_REMAP_W<MAPR_SPEC> {
        TIM1_REMAP_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim2_remap(&mut self) -> TIM2_REMAP_W<MAPR_SPEC> {
        TIM2_REMAP_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim3_remap(&mut self) -> TIM3_REMAP_W<MAPR_SPEC> {
        TIM3_REMAP_W::new(self, 10)
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim4_remap(&mut self) -> TIM4_REMAP_W<MAPR_SPEC> {
        TIM4_REMAP_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn can_remap(&mut self) -> CAN_REMAP_W<MAPR_SPEC> {
        CAN_REMAP_W::new(self, 13)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01_remap(&mut self) -> PD01_REMAP_W<MAPR_SPEC> {
        PD01_REMAP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn tim5ch4_iremap(&mut self) -> TIM5CH4_IREMAP_W<MAPR_SPEC> {
        TIM5CH4_IREMAP_W::new(self, 16)
    }
    #[doc = "Bit 17 - ADC 1 External trigger injected conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etrginj_remap(&mut self) -> ADC1_ETRGINJ_REMAP_W<MAPR_SPEC> {
        ADC1_ETRGINJ_REMAP_W::new(self, 17)
    }
    #[doc = "Bit 18 - ADC 1 external trigger regular conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etrgreg_remap(&mut self) -> ADC1_ETRGREG_REMAP_W<MAPR_SPEC> {
        ADC1_ETRGREG_REMAP_W::new(self, 18)
    }
    #[doc = "Bit 19 - ADC 2 external trigger injected conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_etrginj_remap(&mut self) -> ADC2_ETRGINJ_REMAP_W<MAPR_SPEC> {
        ADC2_ETRGINJ_REMAP_W::new(self, 19)
    }
    #[doc = "Bit 20 - ADC 2 external trigger regular conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_etrgreg_remap(&mut self) -> ADC2_ETRGREG_REMAP_W<MAPR_SPEC> {
        ADC2_ETRGREG_REMAP_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    #[must_use]
    pub fn swj_cfg(&mut self) -> SWJ_CFG_W<MAPR_SPEC> {
        SWJ_CFG_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mapr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mapr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAPR_SPEC;
impl crate::RegisterSpec for MAPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mapr::R`](R) reader structure"]
impl crate::Readable for MAPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mapr::W`](W) writer structure"]
impl crate::Writable for MAPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAPR to value 0"]
impl crate::Resettable for MAPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
