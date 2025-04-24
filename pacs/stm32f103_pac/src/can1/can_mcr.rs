#[doc = "Register `CAN_MCR` reader"]
pub type R = crate::R<CAN_MCR_SPEC>;
#[doc = "Register `CAN_MCR` writer"]
pub type W = crate::W<CAN_MCR_SPEC>;
#[doc = "Field `INRQ` reader - INRQ"]
pub type INRQ_R = crate::BitReader;
#[doc = "Field `INRQ` writer - INRQ"]
pub type INRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - SLEEP"]
pub type SLEEP_R = crate::BitReader;
#[doc = "Field `SLEEP` writer - SLEEP"]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFP` reader - TXFP"]
pub type TXFP_R = crate::BitReader;
#[doc = "Field `TXFP` writer - TXFP"]
pub type TXFP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFLM` reader - RFLM"]
pub type RFLM_R = crate::BitReader;
#[doc = "Field `RFLM` writer - RFLM"]
pub type RFLM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NART` reader - NART"]
pub type NART_R = crate::BitReader;
#[doc = "Field `NART` writer - NART"]
pub type NART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWUM` reader - AWUM"]
pub type AWUM_R = crate::BitReader;
#[doc = "Field `AWUM` writer - AWUM"]
pub type AWUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABOM` reader - ABOM"]
pub type ABOM_R = crate::BitReader;
#[doc = "Field `ABOM` writer - ABOM"]
pub type ABOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTCM` reader - TTCM"]
pub type TTCM_R = crate::BitReader;
#[doc = "Field `TTCM` writer - TTCM"]
pub type TTCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - RESET"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - RESET"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBF` reader - DBF"]
pub type DBF_R = crate::BitReader;
#[doc = "Field `DBF` writer - DBF"]
pub type DBF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - INRQ"]
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SLEEP"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXFP"]
    #[inline(always)]
    pub fn txfp(&self) -> TXFP_R {
        TXFP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RFLM"]
    #[inline(always)]
    pub fn rflm(&self) -> RFLM_R {
        RFLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NART"]
    #[inline(always)]
    pub fn nart(&self) -> NART_R {
        NART_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AWUM"]
    #[inline(always)]
    pub fn awum(&self) -> AWUM_R {
        AWUM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ABOM"]
    #[inline(always)]
    pub fn abom(&self) -> ABOM_R {
        ABOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TTCM"]
    #[inline(always)]
    pub fn ttcm(&self) -> TTCM_R {
        TTCM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DBF"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INRQ"]
    #[inline(always)]
    #[must_use]
    pub fn inrq(&mut self) -> INRQ_W<CAN_MCR_SPEC> {
        INRQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - SLEEP"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<CAN_MCR_SPEC> {
        SLEEP_W::new(self, 1)
    }
    #[doc = "Bit 2 - TXFP"]
    #[inline(always)]
    #[must_use]
    pub fn txfp(&mut self) -> TXFP_W<CAN_MCR_SPEC> {
        TXFP_W::new(self, 2)
    }
    #[doc = "Bit 3 - RFLM"]
    #[inline(always)]
    #[must_use]
    pub fn rflm(&mut self) -> RFLM_W<CAN_MCR_SPEC> {
        RFLM_W::new(self, 3)
    }
    #[doc = "Bit 4 - NART"]
    #[inline(always)]
    #[must_use]
    pub fn nart(&mut self) -> NART_W<CAN_MCR_SPEC> {
        NART_W::new(self, 4)
    }
    #[doc = "Bit 5 - AWUM"]
    #[inline(always)]
    #[must_use]
    pub fn awum(&mut self) -> AWUM_W<CAN_MCR_SPEC> {
        AWUM_W::new(self, 5)
    }
    #[doc = "Bit 6 - ABOM"]
    #[inline(always)]
    #[must_use]
    pub fn abom(&mut self) -> ABOM_W<CAN_MCR_SPEC> {
        ABOM_W::new(self, 6)
    }
    #[doc = "Bit 7 - TTCM"]
    #[inline(always)]
    #[must_use]
    pub fn ttcm(&mut self) -> TTCM_W<CAN_MCR_SPEC> {
        TTCM_W::new(self, 7)
    }
    #[doc = "Bit 15 - RESET"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CAN_MCR_SPEC> {
        RESET_W::new(self, 15)
    }
    #[doc = "Bit 16 - DBF"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<CAN_MCR_SPEC> {
        DBF_W::new(self, 16)
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
#[doc = "CAN_MCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_MCR_SPEC;
impl crate::RegisterSpec for CAN_MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mcr::R`](R) reader structure"]
impl crate::Readable for CAN_MCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_mcr::W`](W) writer structure"]
impl crate::Writable for CAN_MCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_MCR to value 0"]
impl crate::Resettable for CAN_MCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
