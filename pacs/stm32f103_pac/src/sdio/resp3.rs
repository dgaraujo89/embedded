#[doc = "Register `RESP3` reader"]
pub type R = crate::R<RESP3_SPEC>;
#[doc = "Field `CARDSTATUS3` reader - CARDSTATUS3"]
pub type CARDSTATUS3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS3"]
    #[inline(always)]
    pub fn cardstatus3(&self) -> CARDSTATUS3_R {
        CARDSTATUS3_R::new(self.bits)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP3_SPEC;
impl crate::RegisterSpec for RESP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp3::R`](R) reader structure"]
impl crate::Readable for RESP3_SPEC {}
#[doc = "`reset()` method sets RESP3 to value 0"]
impl crate::Resettable for RESP3_SPEC {
    const RESET_VALUE: u32 = 0;
}
