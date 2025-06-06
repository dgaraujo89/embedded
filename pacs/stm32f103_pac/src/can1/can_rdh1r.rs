#[doc = "Register `CAN_RDH1R` reader"]
pub type R = crate::R<CAN_RDH1R_SPEC>;
#[doc = "Field `DATA4` reader - DATA4"]
pub type DATA4_R = crate::FieldReader;
#[doc = "Field `DATA5` reader - DATA5"]
pub type DATA5_R = crate::FieldReader;
#[doc = "Field `DATA6` reader - DATA6"]
pub type DATA6_R = crate::FieldReader;
#[doc = "Field `DATA7` reader - DATA7"]
pub type DATA7_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CAN_RDH1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rdh1r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_RDH1R_SPEC;
impl crate::RegisterSpec for CAN_RDH1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_rdh1r::R`](R) reader structure"]
impl crate::Readable for CAN_RDH1R_SPEC {}
#[doc = "`reset()` method sets CAN_RDH1R to value 0"]
impl crate::Resettable for CAN_RDH1R_SPEC {
    const RESET_VALUE: u32 = 0;
}
