#[doc = "Register `ACTRL` reader"]
pub type R = crate::R<ACTRL_SPEC>;
#[doc = "Register `ACTRL` writer"]
pub type W = crate::W<ACTRL_SPEC>;
#[doc = "Field `DISFOLD` reader - DISFOLD"]
pub type DISFOLD_R = crate::BitReader;
#[doc = "Field `DISFOLD` writer - DISFOLD"]
pub type DISFOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPEXCODIS` reader - FPEXCODIS"]
pub type FPEXCODIS_R = crate::BitReader;
#[doc = "Field `FPEXCODIS` writer - FPEXCODIS"]
pub type FPEXCODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISRAMODE` reader - DISRAMODE"]
pub type DISRAMODE_R = crate::BitReader;
#[doc = "Field `DISRAMODE` writer - DISRAMODE"]
pub type DISRAMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISITMATBFLUSH` reader - DISITMATBFLUSH"]
pub type DISITMATBFLUSH_R = crate::BitReader;
#[doc = "Field `DISITMATBFLUSH` writer - DISITMATBFLUSH"]
pub type DISITMATBFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - DISFOLD"]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - FPEXCODIS"]
    #[inline(always)]
    pub fn fpexcodis(&self) -> FPEXCODIS_R {
        FPEXCODIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DISRAMODE"]
    #[inline(always)]
    pub fn disramode(&self) -> DISRAMODE_R {
        DISRAMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DISITMATBFLUSH"]
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSH_R {
        DISITMATBFLUSH_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DISFOLD"]
    #[inline(always)]
    #[must_use]
    pub fn disfold(&mut self) -> DISFOLD_W<ACTRL_SPEC> {
        DISFOLD_W::new(self, 2)
    }
    #[doc = "Bit 10 - FPEXCODIS"]
    #[inline(always)]
    #[must_use]
    pub fn fpexcodis(&mut self) -> FPEXCODIS_W<ACTRL_SPEC> {
        FPEXCODIS_W::new(self, 10)
    }
    #[doc = "Bit 11 - DISRAMODE"]
    #[inline(always)]
    #[must_use]
    pub fn disramode(&mut self) -> DISRAMODE_W<ACTRL_SPEC> {
        DISRAMODE_W::new(self, 11)
    }
    #[doc = "Bit 12 - DISITMATBFLUSH"]
    #[inline(always)]
    #[must_use]
    pub fn disitmatbflush(&mut self) -> DISITMATBFLUSH_W<ACTRL_SPEC> {
        DISITMATBFLUSH_W::new(self, 12)
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
#[doc = "Auxiliary control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACTRL_SPEC;
impl crate::RegisterSpec for ACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actrl::R`](R) reader structure"]
impl crate::Readable for ACTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`actrl::W`](W) writer structure"]
impl crate::Writable for ACTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACTRL to value 0"]
impl crate::Resettable for ACTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
