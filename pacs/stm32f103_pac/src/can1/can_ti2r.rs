#[doc = "Register `CAN_TI2R` reader"]
pub type R = crate::R<CAN_TI2R_SPEC>;
#[doc = "Register `CAN_TI2R` writer"]
pub type W = crate::W<CAN_TI2R_SPEC>;
#[doc = "Field `TXRQ` reader - TXRQ"]
pub type TXRQ_R = crate::BitReader;
#[doc = "Field `TXRQ` writer - TXRQ"]
pub type TXRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTR` reader - RTR"]
pub type RTR_R = crate::BitReader;
#[doc = "Field `RTR` writer - RTR"]
pub type RTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDE` reader - IDE"]
pub type IDE_R = crate::BitReader;
#[doc = "Field `IDE` writer - IDE"]
pub type IDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXID` reader - EXID"]
pub type EXID_R = crate::FieldReader<u32>;
#[doc = "Field `EXID` writer - EXID"]
pub type EXID_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `STID` reader - STID"]
pub type STID_R = crate::FieldReader<u16>;
#[doc = "Field `STID` writer - STID"]
pub type STID_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    #[must_use]
    pub fn txrq(&mut self) -> TXRQ_W<CAN_TI2R_SPEC> {
        TXRQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<CAN_TI2R_SPEC> {
        RTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IDE_W<CAN_TI2R_SPEC> {
        IDE_W::new(self, 2)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    #[must_use]
    pub fn exid(&mut self) -> EXID_W<CAN_TI2R_SPEC> {
        EXID_W::new(self, 3)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    #[must_use]
    pub fn stid(&mut self) -> STID_W<CAN_TI2R_SPEC> {
        STID_W::new(self, 21)
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
#[doc = "CAN_TI2R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ti2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_ti2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_TI2R_SPEC;
impl crate::RegisterSpec for CAN_TI2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ti2r::R`](R) reader structure"]
impl crate::Readable for CAN_TI2R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_ti2r::W`](W) writer structure"]
impl crate::Writable for CAN_TI2R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_TI2R to value 0"]
impl crate::Resettable for CAN_TI2R_SPEC {
    const RESET_VALUE: u32 = 0;
}
