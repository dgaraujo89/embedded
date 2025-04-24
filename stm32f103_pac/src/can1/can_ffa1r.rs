#[doc = "Register `CAN_FFA1R` reader"]
pub type R = crate::R<CAN_FFA1R_SPEC>;
#[doc = "Register `CAN_FFA1R` writer"]
pub type W = crate::W<CAN_FFA1R_SPEC>;
#[doc = "Field `FFA0` reader - Filter FIFO assignment for filter 0"]
pub type FFA0_R = crate::BitReader;
#[doc = "Field `FFA0` writer - Filter FIFO assignment for filter 0"]
pub type FFA0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA1` reader - Filter FIFO assignment for filter 1"]
pub type FFA1_R = crate::BitReader;
#[doc = "Field `FFA1` writer - Filter FIFO assignment for filter 1"]
pub type FFA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA2` reader - Filter FIFO assignment for filter 2"]
pub type FFA2_R = crate::BitReader;
#[doc = "Field `FFA2` writer - Filter FIFO assignment for filter 2"]
pub type FFA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA3` reader - Filter FIFO assignment for filter 3"]
pub type FFA3_R = crate::BitReader;
#[doc = "Field `FFA3` writer - Filter FIFO assignment for filter 3"]
pub type FFA3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA4` reader - Filter FIFO assignment for filter 4"]
pub type FFA4_R = crate::BitReader;
#[doc = "Field `FFA4` writer - Filter FIFO assignment for filter 4"]
pub type FFA4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA5` reader - Filter FIFO assignment for filter 5"]
pub type FFA5_R = crate::BitReader;
#[doc = "Field `FFA5` writer - Filter FIFO assignment for filter 5"]
pub type FFA5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA6` reader - Filter FIFO assignment for filter 6"]
pub type FFA6_R = crate::BitReader;
#[doc = "Field `FFA6` writer - Filter FIFO assignment for filter 6"]
pub type FFA6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA7` reader - Filter FIFO assignment for filter 7"]
pub type FFA7_R = crate::BitReader;
#[doc = "Field `FFA7` writer - Filter FIFO assignment for filter 7"]
pub type FFA7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA8` reader - Filter FIFO assignment for filter 8"]
pub type FFA8_R = crate::BitReader;
#[doc = "Field `FFA8` writer - Filter FIFO assignment for filter 8"]
pub type FFA8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA9` reader - Filter FIFO assignment for filter 9"]
pub type FFA9_R = crate::BitReader;
#[doc = "Field `FFA9` writer - Filter FIFO assignment for filter 9"]
pub type FFA9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA10` reader - Filter FIFO assignment for filter 10"]
pub type FFA10_R = crate::BitReader;
#[doc = "Field `FFA10` writer - Filter FIFO assignment for filter 10"]
pub type FFA10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA11` reader - Filter FIFO assignment for filter 11"]
pub type FFA11_R = crate::BitReader;
#[doc = "Field `FFA11` writer - Filter FIFO assignment for filter 11"]
pub type FFA11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA12` reader - Filter FIFO assignment for filter 12"]
pub type FFA12_R = crate::BitReader;
#[doc = "Field `FFA12` writer - Filter FIFO assignment for filter 12"]
pub type FFA12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA13` reader - Filter FIFO assignment for filter 13"]
pub type FFA13_R = crate::BitReader;
#[doc = "Field `FFA13` writer - Filter FIFO assignment for filter 13"]
pub type FFA13_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&self) -> FFA0_R {
        FFA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&self) -> FFA1_R {
        FFA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&self) -> FFA2_R {
        FFA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&self) -> FFA3_R {
        FFA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&self) -> FFA4_R {
        FFA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&self) -> FFA5_R {
        FFA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&self) -> FFA6_R {
        FFA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&self) -> FFA7_R {
        FFA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&self) -> FFA8_R {
        FFA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&self) -> FFA9_R {
        FFA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&self) -> FFA10_R {
        FFA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&self) -> FFA11_R {
        FFA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&self) -> FFA12_R {
        FFA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&self) -> FFA13_R {
        FFA13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    #[must_use]
    pub fn ffa0(&mut self) -> FFA0_W<CAN_FFA1R_SPEC> {
        FFA0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    #[must_use]
    pub fn ffa1(&mut self) -> FFA1_W<CAN_FFA1R_SPEC> {
        FFA1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    #[must_use]
    pub fn ffa2(&mut self) -> FFA2_W<CAN_FFA1R_SPEC> {
        FFA2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    #[must_use]
    pub fn ffa3(&mut self) -> FFA3_W<CAN_FFA1R_SPEC> {
        FFA3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    #[must_use]
    pub fn ffa4(&mut self) -> FFA4_W<CAN_FFA1R_SPEC> {
        FFA4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    #[must_use]
    pub fn ffa5(&mut self) -> FFA5_W<CAN_FFA1R_SPEC> {
        FFA5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    #[must_use]
    pub fn ffa6(&mut self) -> FFA6_W<CAN_FFA1R_SPEC> {
        FFA6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    #[must_use]
    pub fn ffa7(&mut self) -> FFA7_W<CAN_FFA1R_SPEC> {
        FFA7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    #[must_use]
    pub fn ffa8(&mut self) -> FFA8_W<CAN_FFA1R_SPEC> {
        FFA8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    #[must_use]
    pub fn ffa9(&mut self) -> FFA9_W<CAN_FFA1R_SPEC> {
        FFA9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    #[must_use]
    pub fn ffa10(&mut self) -> FFA10_W<CAN_FFA1R_SPEC> {
        FFA10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    #[must_use]
    pub fn ffa11(&mut self) -> FFA11_W<CAN_FFA1R_SPEC> {
        FFA11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    #[must_use]
    pub fn ffa12(&mut self) -> FFA12_W<CAN_FFA1R_SPEC> {
        FFA12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    #[must_use]
    pub fn ffa13(&mut self) -> FFA13_W<CAN_FFA1R_SPEC> {
        FFA13_W::new(self, 13)
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
#[doc = "CAN_FFA1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ffa1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_ffa1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_FFA1R_SPEC;
impl crate::RegisterSpec for CAN_FFA1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ffa1r::R`](R) reader structure"]
impl crate::Readable for CAN_FFA1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_ffa1r::W`](W) writer structure"]
impl crate::Writable for CAN_FFA1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_FFA1R to value 0"]
impl crate::Resettable for CAN_FFA1R_SPEC {
    const RESET_VALUE: u32 = 0;
}
