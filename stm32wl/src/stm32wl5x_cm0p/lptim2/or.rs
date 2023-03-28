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
#[doc = "Field `OR_` reader - Option register bit 1"]
pub type OR__R = crate::FieldReader<u8, OR__A>;
#[doc = "Option register bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OR__A {
    #[doc = "0: Input 1 is connected to I/O"]
    Io = 0,
    #[doc = "1: Input 1 is connected to COMP1_OUT"]
    Comp1Out = 1,
    #[doc = "2: Input 1 is connected to COMP2_OUT"]
    Comp2Out = 2,
    #[doc = "3: Input 1 is connected to COMP1_OUT OR COMP2_OUT"]
    OrComp1Comp2 = 3,
}
impl From<OR__A> for u8 {
    #[inline(always)]
    fn from(variant: OR__A) -> Self {
        variant as _
    }
}
impl OR__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OR__A {
        match self.bits {
            0 => OR__A::Io,
            1 => OR__A::Comp1Out,
            2 => OR__A::Comp2Out,
            3 => OR__A::OrComp1Comp2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Io`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == OR__A::Io
    }
    #[doc = "Checks if the value of the field is `Comp1Out`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == OR__A::Comp1Out
    }
    #[doc = "Checks if the value of the field is `Comp2Out`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == OR__A::Comp2Out
    }
    #[doc = "Checks if the value of the field is `OrComp1Comp2`"]
    #[inline(always)]
    pub fn is_or_comp1_comp2(&self) -> bool {
        *self == OR__A::OrComp1Comp2
    }
}
#[doc = "Field `OR_` writer - Option register bit 1"]
pub type OR__W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OR_SPEC, u8, OR__A, 2, O>;
impl<'a, const O: u8> OR__W<'a, O> {
    #[doc = "Input 1 is connected to I/O"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(OR__A::Io)
    }
    #[doc = "Input 1 is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(OR__A::Comp1Out)
    }
    #[doc = "Input 1 is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(OR__A::Comp2Out)
    }
    #[doc = "Input 1 is connected to COMP1_OUT OR COMP2_OUT"]
    #[inline(always)]
    pub fn or_comp1_comp2(self) -> &'a mut W {
        self.variant(OR__A::OrComp1Comp2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Option register bit 1"]
    #[inline(always)]
    pub fn or_(&self) -> OR__R {
        OR__R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Option register bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn or_(&mut self) -> OR__W<0> {
        OR__W::new(self)
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
