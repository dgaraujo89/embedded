#[doc = "Register `SQR3` reader"]
pub type R = crate::R<SQR3_SPEC>;
#[doc = "Register `SQR3` writer"]
pub type W = crate::W<SQR3_SPEC>;
#[doc = "Field `SQ1` reader - 1st conversion in regular sequence"]
pub type SQ1_R = crate::FieldReader;
#[doc = "Field `SQ1` writer - 1st conversion in regular sequence"]
pub type SQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ2` reader - 2nd conversion in regular sequence"]
pub type SQ2_R = crate::FieldReader;
#[doc = "Field `SQ2` writer - 2nd conversion in regular sequence"]
pub type SQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ3` reader - 3rd conversion in regular sequence"]
pub type SQ3_R = crate::FieldReader;
#[doc = "Field `SQ3` writer - 3rd conversion in regular sequence"]
pub type SQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ4` reader - 4th conversion in regular sequence"]
pub type SQ4_R = crate::FieldReader;
#[doc = "Field `SQ4` writer - 4th conversion in regular sequence"]
pub type SQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ5` reader - 5th conversion in regular sequence"]
pub type SQ5_R = crate::FieldReader;
#[doc = "Field `SQ5` writer - 5th conversion in regular sequence"]
pub type SQ5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ6` reader - 6th conversion in regular sequence"]
pub type SQ6_R = crate::FieldReader;
#[doc = "Field `SQ6` writer - 6th conversion in regular sequence"]
pub type SQ6_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq1(&mut self) -> SQ1_W<SQR3_SPEC> {
        SQ1_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq2(&mut self) -> SQ2_W<SQR3_SPEC> {
        SQ2_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq3(&mut self) -> SQ3_W<SQR3_SPEC> {
        SQ3_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq4(&mut self) -> SQ4_W<SQR3_SPEC> {
        SQ4_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq5(&mut self) -> SQ5_W<SQR3_SPEC> {
        SQ5_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq6(&mut self) -> SQ6_W<SQR3_SPEC> {
        SQ6_W::new(self, 25)
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
#[doc = "regular sequence register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR3_SPEC;
impl crate::RegisterSpec for SQR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr3::R`](R) reader structure"]
impl crate::Readable for SQR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sqr3::W`](W) writer structure"]
impl crate::Writable for SQR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SQR3 to value 0"]
impl crate::Resettable for SQR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
