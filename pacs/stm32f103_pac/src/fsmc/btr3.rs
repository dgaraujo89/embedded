#[doc = "Register `BTR3` reader"]
pub type R = crate::R<BTR3_SPEC>;
#[doc = "Register `BTR3` writer"]
pub type W = crate::W<BTR3_SPEC>;
#[doc = "Field `ADDSET` reader - ADDSET"]
pub type ADDSET_R = crate::FieldReader;
#[doc = "Field `ADDSET` writer - ADDSET"]
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDHLD` reader - ADDHLD"]
pub type ADDHLD_R = crate::FieldReader;
#[doc = "Field `ADDHLD` writer - ADDHLD"]
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAST` reader - DATAST"]
pub type DATAST_R = crate::FieldReader;
#[doc = "Field `DATAST` writer - DATAST"]
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSTURN` reader - BUSTURN"]
pub type BUSTURN_R = crate::FieldReader;
#[doc = "Field `BUSTURN` writer - BUSTURN"]
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKDIV` reader - CLKDIV"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - CLKDIV"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATLAT` reader - DATLAT"]
pub type DATLAT_R = crate::FieldReader;
#[doc = "Field `DATLAT` writer - DATLAT"]
pub type DATLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACCMOD` reader - ACCMOD"]
pub type ACCMOD_R = crate::FieldReader;
#[doc = "Field `ACCMOD` writer - ACCMOD"]
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - ADDSET"]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ADDHLD"]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - DATAST"]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - BUSTURN"]
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DATLAT"]
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - ACCMOD"]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDSET"]
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<BTR3_SPEC> {
        ADDSET_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - ADDHLD"]
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<BTR3_SPEC> {
        ADDHLD_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - DATAST"]
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<BTR3_SPEC> {
        DATAST_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - BUSTURN"]
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<BTR3_SPEC> {
        BUSTURN_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - CLKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<BTR3_SPEC> {
        CLKDIV_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - DATLAT"]
    #[inline(always)]
    #[must_use]
    pub fn datlat(&mut self) -> DATLAT_W<BTR3_SPEC> {
        DATLAT_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - ACCMOD"]
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<BTR3_SPEC> {
        ACCMOD_W::new(self, 28)
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
#[doc = "SRAM/NOR-Flash chip-select timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTR3_SPEC;
impl crate::RegisterSpec for BTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr3::R`](R) reader structure"]
impl crate::Readable for BTR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btr3::W`](W) writer structure"]
impl crate::Writable for BTR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTR3 to value 0xffff_ffff"]
impl crate::Resettable for BTR3_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
