#[doc = "Register `I3C_RMR` reader"]
pub struct R(crate::R<I3C_RMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C_RMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C_RMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C_RMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IBIRDCNT` reader - IBI received payload data count (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the number of data bytes effectively received in the I3C_IBIDR register."]
pub type IBIRDCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCODE` reader - received CCC code (when the I3C is configured as target) When the I3C is configured as target, this field logs the received CCC code."]
pub type RCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RADD` reader - received target address (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the received dynamic address from the target during acknowledged IBI or controller-role request."]
pub type RADD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - IBI received payload data count (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the number of data bytes effectively received in the I3C_IBIDR register."]
    #[inline(always)]
    pub fn ibirdcnt(&self) -> IBIRDCNT_R {
        IBIRDCNT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - received CCC code (when the I3C is configured as target) When the I3C is configured as target, this field logs the received CCC code."]
    #[inline(always)]
    pub fn rcode(&self) -> RCODE_R {
        RCODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 17:23 - received target address (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the received dynamic address from the target during acknowledged IBI or controller-role request."]
    #[inline(always)]
    pub fn radd(&self) -> RADD_R {
        RADD_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
#[doc = "I3C received message register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_rmr](index.html) module"]
pub struct I3C_RMR_SPEC;
impl crate::RegisterSpec for I3C_RMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i3c_rmr::R](R) reader structure"]
impl crate::Readable for I3C_RMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I3C_RMR to value 0"]
impl crate::Resettable for I3C_RMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
