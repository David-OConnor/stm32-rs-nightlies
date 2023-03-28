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
#[doc = "Field `OR_0` reader - Option register bit 0"]
pub type OR_0_R = crate::BitReader<OR_0_A>;
#[doc = "Option register bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OR_0_A {
    #[doc = "0: LPTIM1 input 1 is connected to I/O"]
    Io = 0,
    #[doc = "1: LPTIM1 input 1 is connected to COMP1_OUT"]
    Comp1Out = 1,
}
impl From<OR_0_A> for bool {
    #[inline(always)]
    fn from(variant: OR_0_A) -> Self {
        variant as u8 != 0
    }
}
impl OR_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OR_0_A {
        match self.bits {
            false => OR_0_A::Io,
            true => OR_0_A::Comp1Out,
        }
    }
    #[doc = "Checks if the value of the field is `Io`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == OR_0_A::Io
    }
    #[doc = "Checks if the value of the field is `Comp1Out`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == OR_0_A::Comp1Out
    }
}
#[doc = "Field `OR_0` writer - Option register bit 0"]
pub type OR_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, OR_0_A, O>;
impl<'a, const O: u8> OR_0_W<'a, O> {
    #[doc = "LPTIM1 input 1 is connected to I/O"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(OR_0_A::Io)
    }
    #[doc = "LPTIM1 input 1 is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(OR_0_A::Comp1Out)
    }
}
#[doc = "Field `OR_1` reader - Option register bit 1"]
pub type OR_1_R = crate::BitReader<OR_1_A>;
#[doc = "Option register bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OR_1_A {
    #[doc = "0: LPTIM1 input 2 is connected to I/O"]
    Io = 0,
    #[doc = "1: LPTIM1 input 2 is connected to COMP2_OUT"]
    Comp2Out = 1,
}
impl From<OR_1_A> for bool {
    #[inline(always)]
    fn from(variant: OR_1_A) -> Self {
        variant as u8 != 0
    }
}
impl OR_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OR_1_A {
        match self.bits {
            false => OR_1_A::Io,
            true => OR_1_A::Comp2Out,
        }
    }
    #[doc = "Checks if the value of the field is `Io`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == OR_1_A::Io
    }
    #[doc = "Checks if the value of the field is `Comp2Out`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == OR_1_A::Comp2Out
    }
}
#[doc = "Field `OR_1` writer - Option register bit 1"]
pub type OR_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, OR_1_A, O>;
impl<'a, const O: u8> OR_1_W<'a, O> {
    #[doc = "LPTIM1 input 2 is connected to I/O"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(OR_1_A::Io)
    }
    #[doc = "LPTIM1 input 2 is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(OR_1_A::Comp2Out)
    }
}
impl R {
    #[doc = "Bit 0 - Option register bit 0"]
    #[inline(always)]
    pub fn or_0(&self) -> OR_0_R {
        OR_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option register bit 1"]
    #[inline(always)]
    pub fn or_1(&self) -> OR_1_R {
        OR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option register bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn or_0(&mut self) -> OR_0_W<0> {
        OR_0_W::new(self)
    }
    #[doc = "Bit 1 - Option register bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn or_1(&mut self) -> OR_1_W<1> {
        OR_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
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
