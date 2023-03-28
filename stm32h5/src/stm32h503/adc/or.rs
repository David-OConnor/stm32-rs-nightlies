#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR` writer"]
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OP0` reader - Option bit 0"]
pub type OP0_R = crate::BitReader<bool>;
#[doc = "Field `OP0` writer - Option bit 0"]
pub type OP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
#[doc = "Field `OP1` reader - Option bit 1"]
pub type OP1_R = crate::BitReader<bool>;
#[doc = "Field `OP1` writer - Option bit 1"]
pub type OP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Option bit 0"]
    #[inline(always)]
    pub fn op0(&self) -> OP0_R {
        OP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option bit 1"]
    #[inline(always)]
    pub fn op1(&self) -> OP1_R {
        OP1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn op0(&mut self) -> OP0_W<0> {
        OP0_W::new(self)
    }
    #[doc = "Bit 1 - Option bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn op1(&mut self) -> OP1_W<1> {
        OP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or::W](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
