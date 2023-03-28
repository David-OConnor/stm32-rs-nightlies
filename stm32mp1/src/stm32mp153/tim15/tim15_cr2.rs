#[doc = "Register `TIM15_CR2` reader"]
pub struct R(crate::R<TIM15_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM15_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM15_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM15_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM15_CR2` writer"]
pub struct W(crate::W<TIM15_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM15_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TIM15_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM15_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCPC` reader - CCPC"]
pub type CCPC_R = crate::BitReader<bool>;
#[doc = "Field `CCPC` writer - CCPC"]
pub type CCPC_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CR2_SPEC, bool, O>;
#[doc = "Field `CCUS` reader - CCUS"]
pub type CCUS_R = crate::BitReader<bool>;
#[doc = "Field `CCUS` writer - CCUS"]
pub type CCUS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CR2_SPEC, bool, O>;
#[doc = "Field `CCDS` reader - CCDS"]
pub type CCDS_R = crate::BitReader<bool>;
#[doc = "Field `CCDS` writer - CCDS"]
pub type CCDS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CR2_SPEC, bool, O>;
#[doc = "Field `MMS` reader - MMS"]
pub type MMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMS` writer - MMS"]
pub type MMS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM15_CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `TI1S` reader - TI1S"]
pub type TI1S_R = crate::BitReader<bool>;
#[doc = "Field `TI1S` writer - TI1S"]
pub type TI1S_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CR2_SPEC, bool, O>;
#[doc = "Field `OIS1` reader - OIS1"]
pub type OIS1_R = crate::BitReader<bool>;
#[doc = "Field `OIS1` writer - OIS1"]
pub type OIS1_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CR2_SPEC, bool, O>;
#[doc = "Field `OIS1N` reader - OIS1N"]
pub type OIS1N_R = crate::BitReader<bool>;
#[doc = "Field `OIS1N` writer - OIS1N"]
pub type OIS1N_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CR2_SPEC, bool, O>;
#[doc = "Field `OIS2` reader - OIS2"]
pub type OIS2_R = crate::BitReader<bool>;
#[doc = "Field `OIS2` writer - OIS2"]
pub type OIS2_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CCPC"]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CCUS"]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCDS"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - MMS"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1S"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OIS1"]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OIS1N"]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OIS2"]
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCPC"]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<0> {
        CCPC_W::new(self)
    }
    #[doc = "Bit 2 - CCUS"]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<2> {
        CCUS_W::new(self)
    }
    #[doc = "Bit 3 - CCDS"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<3> {
        CCDS_W::new(self)
    }
    #[doc = "Bits 4:6 - MMS"]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<4> {
        MMS_W::new(self)
    }
    #[doc = "Bit 7 - TI1S"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<7> {
        TI1S_W::new(self)
    }
    #[doc = "Bit 8 - OIS1"]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<8> {
        OIS1_W::new(self)
    }
    #[doc = "Bit 9 - OIS1N"]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<9> {
        OIS1N_W::new(self)
    }
    #[doc = "Bit 10 - OIS2"]
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> OIS2_W<10> {
        OIS2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM15 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_cr2](index.html) module"]
pub struct TIM15_CR2_SPEC;
impl crate::RegisterSpec for TIM15_CR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim15_cr2::R](R) reader structure"]
impl crate::Readable for TIM15_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim15_cr2::W](W) writer structure"]
impl crate::Writable for TIM15_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM15_CR2 to value 0"]
impl crate::Resettable for TIM15_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
